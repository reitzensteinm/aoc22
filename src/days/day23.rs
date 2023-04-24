use std::fs::read_to_string;

// With the exception of using an intermediate mask that represents surrounding dwarves,
// this is a pretty straight forward implementation of the stated rules.
// The field is treated as sparse. A list of dwarves is kept, and the algorithm jumps to
// the affected squares. The dwarves also sleep where it is appropriate, speeding up processing.
// I'm a little sad this didn't go faster - 1000 rounds takes 42 milliseconds on a 3900X.
// To speed it up, it either needs lower level SIMD magic, or it needs something like HashLife.

struct Dwarf {
    dest: Option<(usize, usize)>,
    pos: (usize, usize),
    awake: bool,
}

#[derive(Copy, Clone, PartialEq)]
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
    dwarves: Vec<Dwarf>,
}

impl Board {
    fn step(&mut self) {
        const CHECKS: [((isize, isize), u8); 4] = [
            ((0, -1), 0b111),
            ((0, 1), 0b11100000),
            ((-1, 0), 0b00101001),
            ((1, 0), 0b10010100),
        ];

        for d in 0..self.dwarves.len() {
            if !self.dwarves[d].awake {
                continue;
            }

            let (x, y) = self.dwarves[d].pos;

            let mut dwarf_map: u8 = 0;

            // Maps surrounding squares into a mask, which can easily (?) be used to figure out what
            // to do next. Manually inlining yields the desired speedup, but I'm curious what
            // matches!() is going to compile down to. Not breaking out Godbolt for this...

            if matches!(self.squares[(y - 1) * self.width + x - 1], Square::Dwarf(_)) {
                dwarf_map |= 1;
            }

            if matches!(self.squares[(y - 1) * self.width + x], Square::Dwarf(_)) {
                dwarf_map |= 1 << 1;
            }

            if matches!(self.squares[(y - 1) * self.width + x + 1], Square::Dwarf(_)) {
                dwarf_map |= 1 << 2;
            }

            if matches!(self.squares[(y) * self.width + x - 1], Square::Dwarf(_)) {
                dwarf_map |= 1 << 3;
            }

            if matches!(self.squares[(y) * self.width + x + 1], Square::Dwarf(_)) {
                dwarf_map |= 1 << 4;
            }

            if matches!(self.squares[(y + 1) * self.width + x - 1], Square::Dwarf(_)) {
                dwarf_map |= 1 << 5;
            }

            if matches!(self.squares[(y + 1) * self.width + x], Square::Dwarf(_)) {
                dwarf_map |= 1 << 6;
            }

            if matches!(self.squares[(y + 1) * self.width + x + 1], Square::Dwarf(_)) {
                dwarf_map |= 1 << 7;
            }

            if dwarf_map == 0b00000000 {
                self.dwarves[d].awake = false;
                continue;
            }

            for cn in 0..4 {
                let (dir, mask) = CHECKS[(cn + self.round) % 4];

                if dwarf_map & mask == 0 {
                    let tx = (x as isize + dir.0) as usize;
                    let ty = (y as isize + dir.1) as usize;
                    let to = self.squares[ty * self.width + tx];

                    if to == Square::Marked {
                        self.squares[ty * self.width + tx] = Square::OverMarked;
                    } else if to == Square::Empty {
                        self.squares[ty * self.width + tx] = Square::Marked;
                    }

                    self.dwarves[d].dest = Some((tx, ty));

                    break;
                }
            }
        }

        let mut any_moved = false;

        for d in 0..self.dwarves.len() {
            if let Some((dx, dy)) = self.dwarves[d].dest {
                let (px, py) = self.dwarves[d].pos;

                let dest_square = dy * self.width + dx;
                let current_square = py * self.width + px;

                if self.squares[dest_square] == Square::OverMarked {
                    self.squares[dest_square] = Square::Empty;
                } else if self.squares[dest_square] == Square::Marked {
                    any_moved = true;
                    self.moved += 1;
                    self.squares[current_square] = Square::Empty;
                    self.squares[dest_square] = Square::Dwarf(d as u16);

                    for cx in -1..=1 {
                        for cy in -1..=1 {
                            if let Square::Dwarf(od) = self.squares[((dy as isize + cy) as usize)
                                * self.width
                                + (dx as isize + cx) as usize]
                            {
                                self.dwarves[od as usize].awake = true;
                            }
                        }
                    }
                    self.dwarves[d].pos = (dx, dy);
                }

                self.dwarves[d].dest = None;
            }
        }

        self.finished = !any_moved;
        self.round += 1;
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

const PADDING: usize = 60;

pub fn day_23() -> (String, String) {
    let f = read_to_string("input/day23.txt").unwrap();

    let mut in_squares = vec![];
    let mut dwarves = vec![];
    let width = PADDING * 2 + 74;
    let height = PADDING * 2 + 74;

    for (y, l) in f.lines().enumerate() {
        in_squares.append(&mut vec![Square::Empty; PADDING]);
        for (x, c) in l.chars().enumerate() {
            in_squares.push(if c == '.' {
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
