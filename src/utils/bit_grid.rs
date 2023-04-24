// This file implements an arbitrarily sized 2D bit grid with operations, such as shift, and,
// or, window, wrapping shift etc.

// It uses references to ensure zero copying, which is a nice ideal. However actually using the
// library is a massive pain. Also, you can in many cases end up going accidentally exponential,
// so it's probably not an actual win. If I did it again, I'd just make it copy, even if high
// performance was the goal.

// Also there are known bugs. But see above. Don't do this.

type BitWidth = u128;
const WIDTH: isize = 128;
const WIDTH_U: usize = 128;

#[derive(Clone)]
pub struct BitGrid {
    backing: Vec<Vec<BitWidth>>,
    width: usize,
    height: usize,
}

pub trait BitView {
    fn get_backing(&self, x: isize, y: isize) -> BitWidth;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

impl BitView for BitGrid {
    fn get_backing(&self, x: isize, y: isize) -> BitWidth {
        let get_backing = |xi: isize| {
            *self
                .backing
                .get(y as usize)
                .map(|v| v.get(xi as usize).unwrap_or(&0))
                .unwrap_or(&0)
        };

        let rem = x.rem_euclid(WIDTH);

        let prev = get_backing((x + WIDTH) / WIDTH - 1);
        let this = get_backing((x + WIDTH) / WIDTH);

        let prev_bits = prev.overflowing_shr(rem as u32).0;
        let this_bits = match this.overflowing_shl((WIDTH - rem) as u32) {
            (_, true) => 0,
            (v, _) => v,
        };

        prev_bits | this_bits
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

pub struct Window<'grid, T>
where
    T: BitView,
{
    view: &'grid T,
    x: isize,
    y: isize,
    width: isize,
    height: isize,
}

impl<'grid, T> Window<'grid, T>
where
    T: BitView,
{
    pub fn new(view: &'grid T, x: isize, y: isize, width: isize, height: isize) -> Self {
        Window {
            view,
            x,
            y,
            width,
            height,
        }
    }
}

impl<T> BitView for Window<'_, T>
where
    T: BitView,
{
    fn get_backing(&self, x: isize, y: isize) -> BitWidth {
        if y < 0 || y >= self.height() as isize {
            return 0;
        }
        let bit_count = self.width - x;
        let mask = match (1 as BitWidth).overflowing_shl(bit_count as u32) {
            (v, false) => v - 1,
            _ => !0,
        };
        self.view.get_backing(x + self.x, y + self.y) & mask
    }

    fn width(&self) -> usize {
        self.width as usize
    }

    fn height(&self) -> usize {
        self.height as usize
    }
}

pub struct Shifted<'grid, T>
where
    T: BitView,
{
    view: &'grid T,
    x: isize,
    y: isize,
}

impl<'grid, T> Shifted<'grid, T>
where
    T: BitView,
{
    pub fn new(view: &'grid T, x: isize, y: isize) -> Self {
        Shifted { view, x, y }
    }
}

impl<T> BitView for Shifted<'_, T>
where
    T: BitView,
{
    fn get_backing(&self, x: isize, y: isize) -> BitWidth {
        if y < 0 || y >= self.height() as isize {
            return 0;
        }
        let bit_count = self.view.width() as isize - x;
        let mask = match (1 as BitWidth).overflowing_shl(bit_count as u32) {
            (v, false) => v - 1,
            _ => !0,
        };
        self.view.get_backing(x - self.x, y - self.y) & mask
    }

    fn width(&self) -> usize {
        self.view.width()
    }

    fn height(&self) -> usize {
        self.view.height()
    }
}

pub struct ShiftedWrap<'grid, T>
where
    T: BitView,
{
    view: &'grid T,
    x: isize,
    y: isize,
}

impl<'grid, T> ShiftedWrap<'grid, T>
where
    T: BitView,
{
    pub fn new(view: &'grid T, x: isize, y: isize) -> Self {
        ShiftedWrap { view, x, y }
    }
}

impl<T> BitView for ShiftedWrap<'_, T>
where
    T: BitView,
{
    fn get_backing(&self, x: isize, y: isize) -> BitWidth {
        if y < 0 || y >= self.height() as isize {
            return 0;
        }
        let width = self.view.width() as isize;
        let height = self.view.height() as isize;
        let a = Shifted::new(self.view, self.x.rem_euclid(width), 0);
        let b = Shifted::new(self.view, self.x.rem_euclid(width) - width, 0);

        Prim::new(&a, &b, |a, b| a | b).get_backing(x, (y - self.y).rem_euclid(height))
    }

    fn width(&self) -> usize {
        self.view.width()
    }

    fn height(&self) -> usize {
        self.view.height()
    }
}

pub struct Prim<'grid, Ta, Tb>
where
    Ta: BitView,
    Tb: BitView,
{
    a: &'grid Ta,
    b: &'grid Tb,
    prim: fn(BitWidth, BitWidth) -> BitWidth,
}

impl<'grid, Ta, Tb> Prim<'grid, Ta, Tb>
where
    Ta: BitView,
    Tb: BitView,
{
    pub fn new(a: &'grid Ta, b: &'grid Tb, prim: fn(BitWidth, BitWidth) -> BitWidth) -> Self {
        Prim { a, b, prim }
    }
}

impl<Ta, Tb> BitView for Prim<'_, Ta, Tb>
where
    Ta: BitView,
    Tb: BitView,
{
    fn get_backing(&self, x: isize, y: isize) -> BitWidth {
        (self.prim)(self.a.get_backing(x, y), self.b.get_backing(x, y))
    }

    fn width(&self) -> usize {
        self.a.width().max(self.b.width())
    }

    fn height(&self) -> usize {
        self.a.height().max(self.b.height())
    }
}

pub struct Or<'grid, Ta, Tb>
where
    Ta: BitView,
    Tb: BitView,
{
    a: &'grid Ta,
    b: &'grid Tb,
}

impl<'grid, Ta, Tb> Or<'grid, Ta, Tb>
where
    Ta: BitView,
    Tb: BitView,
{
    pub fn new(a: &'grid Ta, b: &'grid Tb) -> Self {
        Or { a, b }
    }
}

impl<Ta, Tb> BitView for Or<'_, Ta, Tb>
where
    Ta: BitView,
    Tb: BitView,
{
    fn get_backing(&self, x: isize, y: isize) -> BitWidth {
        self.a.get_backing(x, y) | self.b.get_backing(x, y)
    }

    fn width(&self) -> usize {
        self.a.width().max(self.b.width())
    }

    fn height(&self) -> usize {
        self.a.height().max(self.b.height())
    }
}

impl BitGrid {
    pub fn new(width: usize, height: usize) -> BitGrid {
        BitGrid {
            backing: vec![vec![0; (width + WIDTH_U - 1) / WIDTH_U]; height],
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        (self.backing[y][x / WIDTH_U] >> (x % WIDTH_U) & 1) == 1
    }

    pub fn set(&mut self, x: usize, y: usize, v: bool) {
        if v {
            self.backing[y][x / WIDTH_U] |= 1 << (x % WIDTH_U)
        } else {
            self.backing[y][x / WIDTH_U] &= !(1 << (x % WIDTH_U))
        }
    }

    pub fn from_view(view: &dyn BitView) -> BitGrid {
        let mut backing = vec![];

        for y in 0..view.height() {
            let mut line = vec![];
            for x in 0..(view.width() + (WIDTH_U - 1)) / WIDTH_U {
                line.push(view.get_backing(x as isize * WIDTH, y as isize));
            }
            backing.push(line);
        }

        BitGrid {
            backing,
            width: view.width(),
            height: view.height(),
        }
    }

    #[allow(unused)]
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) {
                    print!("x");
                } else {
                    print!(".");
                }
            }
            println!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;

    #[test]
    fn window_clamp() {
        let bit_test = |x_set, backing_x, window_x, window_size| {
            let mut b = BitGrid::new(128, 1);
            b.set(x_set, 0, true);
            let w = Window::new(&b, window_x, 0, window_size, 1);
            let v = BitGrid::from_view(&w);
            v.print();
            v.get_backing(backing_x, 0)
        };

        assert_eq!(bit_test(3, 0, 0, 2), 0);
        assert_eq!(bit_test(3, 0, 0, WIDTH), 1 << 3);
        assert_eq!(bit_test(WIDTH_U + 3, WIDTH, 0, WIDTH + 1), 0);
        assert_eq!(bit_test(WIDTH_U + 3, 0, 4, WIDTH), 1 << 63);
    }

    // Visual Test
    fn repeat() {
        let mut b = BitGrid::new(16, 4);
        b.set(1, 1, true);

        let mut w = Window::new(&b, 1, 1, 14, 2);

        for n in 0..32 {
            let s = ShiftedWrap::new(&w, n, n);
            let nb = BitGrid::from_view(&s);

            let ow = Window::new(&nb, -1, -1, 16, 4);

            let db = BitGrid::from_view(&ow);

            db.print();
            println!();
        }
    }

    // Visual Test
    fn repeat_2() {
        {
            let mut b = BitGrid::new(16, 1);
            b.set(1, 0, true);
            b.set(2, 0, true);

            for n in 0..32 {
                let w = Window::new(&b, 1, 0, 15, 1);
                let s = ShiftedWrap::new(&w, n, 0);
                let v = BitGrid::from_view(&s);
                v.print();
            }
        }

        {
            let mut b = BitGrid::new(16, 4);
            b.set(0, 1, true);
            b.set(0, 2, true);

            for n in 0..32 {
                let w = Window::new(&b, 0, 0, 16, 3);
                let s = ShiftedWrap::new(&w, 0, n);
                let v = BitGrid::from_view(&s);
                println!();
                v.print();
            }
        }
    }

    // Visual Test
    fn bit_grid_shift() {
        let mut b = BitGrid::new(128, 16);
        b.set(63, 5, true);
        b.print();

        for x in 0..1000 {
            std::thread::sleep(Duration::from_millis(100));

            let sa = Shifted::new(&b, 1, 0);
            let sb = Shifted::new(&b, -1, 0);
            let sc = Shifted::new(&b, 0, -1);
            let sd = Shifted::new(&b, 0, 1);

            let acc = Or::new(&b, &sa);
            let acc = Or::new(&acc, &sb);
            let acc = Or::new(&acc, &sc);
            let acc = Or::new(&acc, &sd);

            b = BitGrid::from_view(&acc);

            b.print();
        }
    }
}
