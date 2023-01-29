pub struct BitGrid {
    backing: Vec<Vec<u64>>,
    width: usize,
    height: usize,
}

pub trait BitView {
    fn get_backing(&self, x: isize, y: isize) -> u64;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

impl BitView for BitGrid {
    fn get_backing(&self, x: isize, y: isize) -> u64 {
        let get_backing = |xi: isize| {
            *self
                .backing
                .get(y as usize)
                .map(|v| v.get(xi as usize).unwrap_or(&0))
                .unwrap_or(&0)
        };
        //
        // println!("Getting backing {} {}", x, y);
        //
        // println!("slots {} {}", (x + 63) / 64 - 1, (x + 63) / 64);

        let rem = x.rem_euclid(64);

        // println!("rem {}", x.rem_euclid(64));

        let prev = get_backing((x + 64) / 64 - 1);
        let this = get_backing((x + 64) / 64);

        let prev_bits = prev.overflowing_shr(rem as u32).0;
        let this_bits = match this.overflowing_shl((64 - rem) as u32) {
            (_, true) => 0,
            (v, _) => v,
        };

        // println!("Bit patterns {:b} {:b}", prev_bits, this_bits);

        prev_bits | this_bits
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

struct Shifted<'grid, T>
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
    fn new(view: &'grid T, x: isize, y: isize) -> Self {
        Shifted { view, x, y }
    }
}

impl<T> BitView for Shifted<'_, T>
where
    T: BitView,
{
    fn get_backing(&self, x: isize, y: isize) -> u64 {
        self.view.get_backing(x - self.x, y - self.y)
    }

    fn width(&self) -> usize {
        self.view.width()
    }

    fn height(&self) -> usize {
        self.view.height()
    }
}

struct Or<'grid, Ta, Tb>
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
    fn new(a: &'grid Ta, b: &'grid Tb) -> Self {
        Or { a, b }
    }
}

impl<Ta, Tb> BitView for Or<'_, Ta, Tb>
where
    Ta: BitView,
    Tb: BitView,
{
    fn get_backing(&self, x: isize, y: isize) -> u64 {
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
            backing: vec![vec![0; (width + 63) / 64]; height],
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        (self.backing[y][x / 64] >> x % 64 & 1) == 1
    }

    pub fn set(&mut self, x: usize, y: usize, v: bool) {
        if v {
            self.backing[y][x / 64] |= 1 << (x % 64)
        } else {
            self.backing[y][x / 64] &= !(1 << (x % 64))
        }
    }

    pub fn from_view(view: &dyn BitView) -> BitGrid {
        let mut backing = vec![];

        for y in 0..view.height() {
            let mut line = vec![];
            for x in 0..(view.width() + 63) / 64 {
                line.push(view.get_backing(x as isize * 64, y as isize));
            }
            backing.push(line);
        }

        BitGrid {
            backing,
            width: view.width(),
            height: view.height(),
        }
    }

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
    fn bit_grid() {
        let mut b = BitGrid::new(64, 64);
        b.set(5, 5, true);
        //b.print();
    }

    #[test]
    fn bit_grid_init() {
        let mut b = BitGrid::new(128, 1);
        b.set(127, 0, true);

        for n in 0..128 {
            let shiftx1 = Shifted::new(&b, -n, 0);
            BitGrid::from_view(&shiftx1).print();
        }

        return;

        // assert_eq!(b.get_backing(0, 1), 0b000000);

        // let shifty = Shifted::new(&b, 0, 1);
        let shiftx1 = Shifted::new(&b, -1, 0);
        let shiftx2 = Shifted::new(&b, 1, 0);
        //
        // BitGrid::from_view(&shifty).print();
        //
        // let fv = BitGrid::from_view(&shiftx);
        // let or = Or::new(&fv, &b);
        // BitGrid::from_view(&or).print();
        //
        // assert_eq!(shifty.get_backing(0, 1), 0b100000);

        // shiftx1.get_backing(0, 0);
        // shiftx2.get_backing(0, 0);
        //
        // assert_eq!(shiftx1.get_backing(0, 0), 0b10000);
        // assert_eq!(shiftx2.get_backing(0, 0), 0b1000000);

        BitGrid::from_view(&shiftx1).print();
        BitGrid::from_view(&shiftx2).print();
    }

    #[test]
    fn bit_grid_shift() {
        let mut b = BitGrid::new(128, 16);
        b.set(63, 5, true);
        b.print();

        for x in 0..1000 {
            std::thread::sleep(Duration::from_millis(100));

            // for y in 0..100 {
            //     println!();
            // }

            let sa = Shifted::new(&b, 1, 0);
            let sb = Shifted::new(&b, -1, 0);
            let sc = Shifted::new(&b, 0, -1);
            let sd = Shifted::new(&b, 0, 1);

            let acc = Or::new(&b, &sa);
            let acc = Or::new(&acc, &sb);
            let acc = Or::new(&acc, &sc);
            let acc = Or::new(&acc, &sd);

            //            let or = Or::new(&b, &shift);

            b = BitGrid::from_view(&acc);

            //
            // b = BitGrid::from_view(
            //     Or::new(&b, &shift)
            // );

            b.print();
        }
    }
}
