use std::fs::read_to_string;

// With the exception of using an intermediate mask that represents surrounding dwarves,
// this is a pretty straight forward implementation of the stated rules.
// The field is treated as sparse. A list of dwarves is kept, and the algorithm jumps to
// the affected squares. The dwarves also sleep where it is appropriate, speeding up processing.
// I'm a little sad this didn't go faster - 1000 rounds takes 42 milliseconds on a 3900X.
// To speed it up, it either needs lower level SIMD magic, or it needs something like HashLife.

#[derive(Copy, Clone, Debug)]
enum Intent {
    Empty,
    One(u16),
    Two(u16, u16),
}

struct Dwarf {
    dest: Option<(usize, usize)>,
    pos: (usize, usize),
    awake: bool,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Square {
    Empty,
    Dwarf(u16),
    Marked,
    OverMarked,
}

struct Board {
    width: usize,
    height: usize,
    round: usize,
    finished: bool,
    moved: usize,
    squares: Vec<Square>,
    intent: Vec<Vec<Intent>>,
    dwarves: Vec<Dwarf>,
}

impl Board {
    fn clear_itent(&mut self, d: usize) {
        let dwarf = &mut self.dwarves[d as usize];
        let (x, y) = dwarf.pos;

        const DIFF: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for r in 0..4 {
            for (dx, dy) in DIFF {
                let ind = (y as isize + dy) * self.width as isize + (x as isize + dx);
                let i = &mut self.intent[r][ind as usize];

                match i {
                    Intent::Empty => {}
                    Intent::One(a) => {
                        if *a as usize == d {
                            *i = Intent::Empty;
                        }
                    }
                    Intent::Two(a, b) => {
                        if *a as usize == d {
                            *i = Intent::One(*b);
                        } else if *b as usize == d {
                            *i = Intent::One(*a);
                        }
                    }
                }
            }
        }
    }

    fn refresh_intent(&mut self, d: usize) {
        self.clear_itent(d);

        let dwarf = &mut self.dwarves[d as usize];
        let (x, y) = dwarf.pos;

        const CHECKS: [((isize, isize), u8); 4] = [
            ((0, -1), 0b111),
            ((0, 1), 0b11100000),
            ((-1, 0), 0b00101001),
            ((1, 0), 0b10010100),
        ];

        let mut dwarf_map: u8 = 0;

        // Maps surrounding squares into a mask, which can easily (?) be used to figure out what
        // to do next. Manually inlining yields the desired speedup, but I'm curious what
        // matches!() is going to compile down to. Not breaking out Godbolt for this...

        if matches!(
            self.squares[((y - 1) * self.width + x - 1)],
            Square::Dwarf(_)
        ) {
            dwarf_map |= 1;
        }

        if matches!(self.squares[((y - 1) * self.width + x)], Square::Dwarf(_)) {
            dwarf_map |= 1 << 1;
        }

        if matches!(
            self.squares[((y - 1) * self.width + x + 1)],
            Square::Dwarf(_)
        ) {
            dwarf_map |= 1 << 2;
        }

        if matches!(self.squares[((y) * self.width + x - 1)], Square::Dwarf(_)) {
            dwarf_map |= 1 << 3;
        }

        if matches!(self.squares[((y) * self.width + x + 1)], Square::Dwarf(_)) {
            dwarf_map |= 1 << 4;
        }

        if matches!(
            self.squares[((y + 1) * self.width + x - 1)],
            Square::Dwarf(_)
        ) {
            dwarf_map |= 1 << 5;
        }

        if matches!(self.squares[((y + 1) * self.width + x)], Square::Dwarf(_)) {
            dwarf_map |= 1 << 6;
        }

        if matches!(
            self.squares[((y + 1) * self.width + x + 1)],
            Square::Dwarf(_)
        ) {
            dwarf_map |= 1 << 7;
        }

        if dwarf_map == 0 {
            return;
        }

        for stage in 0..4 {
            for cn in 0..4 {
                let (dir, mask) = CHECKS[(cn + stage) % 4];

                if dwarf_map & mask == 0 {
                    let tx = (x as isize + dir.0) as usize;
                    let ty = (y as isize + dir.1) as usize;

                    let iq = &mut self.intent[stage][ty * self.width + tx];

                    match *iq {
                        Intent::Empty => {
                            *iq = Intent::One(d as u16);
                        }
                        Intent::One(a) => {
                            if a != d as u16 {
                                *iq = Intent::Two(a, d as u16);
                            }
                        }
                        Intent::Two(a, b) => {
                            if a != d as u16 && b != d as u16 {
                                panic!();
                            }
                        }
                    }

                    break;
                }
            }
        }
    }

    fn step(&mut self) {
        //self.intent = vec![vec![Intent::Empty; self.squares.len()]; 4];

        for d in 0..self.dwarves.len() {
            self.refresh_intent(d);
        }

        let mut any_moved = false;

        let r = self.round % 4;
        for c in 0..self.intent[r].len() {
            if let Intent::One(d) = self.intent[r][c] {
                self.clear_itent(d as usize);
                let (px, py) = self.dwarves[d as usize].pos;
                let current_square = py * self.width + px;
                let dest_square = ((c % self.width), (c / self.width));

                any_moved = true;
                self.moved += 1;
                self.squares[current_square] = Square::Empty;
                self.squares[c] = Square::Dwarf(d as u16);
                self.dwarves[d as usize].pos = dest_square;
            }
        }

        self.finished = !any_moved;
        self.round += 1;

        // println!();
        // for y in 0..self.height {
        //     println!();
        //     for x in 0..self.width {
        //         let chr = if let Square::Dwarf(_) = self.squares[y * self.width + x] {
        //             "x"
        //         } else {
        //             "."
        //         };
        //         print!("{}", chr);
        //     }
        // }
    }

    fn score(&self) -> usize {
        let mut low = (self.width - 1, self.height - 1);
        let mut high = (0, 0);

        for y in 0..self.height {
            for x in 0..self.width {
                let sq = self.squares[y * self.width + x];
                if let Square::Dwarf(_) = sq {
                    low.0 = low.0.min(x);
                    low.1 = low.1.min(y);
                    high.0 = high.0.max(x);
                    high.1 = high.1.max(y);
                }
            }
        }

        let mut score = 0;

        for y in low.1..=high.1 {
            for x in low.0..=high.0 {
                let sq = self.squares[y * self.width + x];
                if let Square::Empty = sq {
                    score += 1;
                }
            }
        }
        score
    }
}

const PADDING: usize = 60; //1 //60;

pub fn day_23() -> (String, String) {
    let f = read_to_string("input/day23.txt").unwrap();

    let lines: Vec<Vec<char>> = f.lines().map(|v| v.chars().collect()).collect();

    let mut in_squares = vec![];
    let mut dwarves = vec![];
    let width = lines[0].len() + PADDING * 2;
    let height = lines.len() + PADDING * 2;

    for (y, l) in lines.iter().enumerate() {
        in_squares.append(&mut vec![Square::Empty; PADDING]);
        for (x, c) in l.iter().enumerate() {
            in_squares.push(if *c == '.' {
                Square::Empty
            } else {
                let num = dwarves.len() as u16;
                dwarves.push(Dwarf {
                    dest: None,
                    awake: true,
                    pos: (PADDING + x, PADDING + y),
                });
                Square::Dwarf(num)
            });
        }
        in_squares.append(&mut vec![Square::Empty; PADDING]);
    }

    let mut squares = vec![Square::Empty; width * PADDING];
    squares.append(&mut in_squares);
    squares.append(&mut vec![Square::Empty; width * PADDING]);

    let mut b = Board {
        width,
        height,
        intent: vec![vec![Intent::Empty; squares.len()]; 4],
        round: 0,
        moved: 0,
        finished: false,
        squares,
        dwarves,
    };

    for _ in 0..10 {
        b.step();
    }

    let part_a = b.score();

    for _ in 0..1000 {
        b.step();
        if b.finished {
            break;
        }
    }

    (format!("{part_a}"), format!("{}", b.round))
}
