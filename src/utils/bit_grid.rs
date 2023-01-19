pub struct BitGrid {
    backing: Vec<Vec<u64>>,
    width: usize,
    height: usize
}

trait BitView {
    fn get_backing(&self,x:isize,y:isize) -> u64;
}

impl BitView for BitGrid {
    fn get_backing(&self, x: isize, y: isize) -> u64 {
        self.backing[y as usize][x as usize / 64]
    }
}

struct Shifted<'grid, T> where T:BitView {
    view: &'grid T,
    x:isize,
    y:isize,
}

impl<T> BitView for Shifted<'_, T> where T :BitView {
    fn get_backing(&self, x: isize, y: isize) -> u64 {
        self.view.get_backing(x+self.x,y+self.y)
    }
}

impl BitGrid {
    pub fn new(width:usize,height:usize) -> BitGrid {
        BitGrid {
            backing: vec![vec![0;(width + 63)/64];height],
            width,height
        }
    }

    pub fn get(&self, x:usize, y:usize) -> bool {
        ( self.backing[y][x / 64] >> x % 64 & 1 ) == 1
    }

    pub fn set(&mut self, x:usize,y:usize, v:bool) {
        if v {
            self.backing[y][x / 64] |= 1 << (x%64)
        } else {
            self.backing[y][x / 64] &= !(1 << (x%64))
        }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x,y) {
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
    #[test]
    fn bit_grid() {
        let mut b = BitGrid::new(64,64);
        b.set(5,5,true);
        b.print();
    }

    #[test]
    fn bit_grid_init() {
        let mut b = BitGrid::new(64,64);
        b.set(5,0,true);
        let s = Shifted {
            view: &b,
            x: 0,
            y: -1
        };

        assert_eq!(s.get_backing(0,1), 0b100000);
        assert_eq!(b.get_backing(0,1), 0b000000);

    }
}

