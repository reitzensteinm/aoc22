use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Clone)]
struct Control {
    dirs: Vec<isize>,
    current: usize,
}

#[derive(Clone)]
struct Field {
    trimmed: usize,
    rows: Vec<u64>,
}

#[derive(Clone)]
struct Piece {
    lines: Vec<u64>,
}

#[derive(Clone)]
struct Game {
    pieces: Vec<Piece>,
    field: Field,
    control: Control,
    piece_count: usize,
}

impl Control {
    fn next(&mut self) -> isize {
        let i = self.dirs[self.current];
        self.current = (self.current + 1) % self.dirs.len();
        i
    }
}

impl Field {
    // Removes everything below a solid line, and stores how many rows were removed
    fn trim(&mut self) {
        if self.rows.len() >= 5 {
            for y in (0..self.rows.len()).rev() {
                let r = self.rows[y];

                if r == 0b1111111 {
                    self.rows.drain(..y);
                    self.trimmed += y;
                    return;
                }
            }
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        Field {
            rows: vec![],
            trimmed: 0,
        }
    }
}

impl Piece {
    fn default_pieces() -> Vec<Piece> {
        vec![
            Piece {
                lines: vec![0b1111000],
            },
            Piece {
                lines: vec![0b0100000, 0b1110000, 0b0100000],
            },
            Piece {
                lines: vec![0b0010000, 0b0010000, 0b1110000],
            },
            Piece {
                lines: vec![0b1000000, 0b1000000, 0b1000000, 0b1000000],
            },
            Piece {
                lines: vec![0b1100000, 0b1100000],
            },
        ]
    }

    fn collides_with_field(&self, field: &Field, (x, y): (isize, isize)) -> bool {
        for (py, l) in self.lines.iter().enumerate() {
            if ((l >> x) & 0b1111111).count_ones() != l.count_ones() {
                return true;
            }

            let yw = y - py as isize;
            if yw < 0 {
                return true;
            }
            if yw < field.rows.len() as isize {
                if (field.rows[yw as usize] & (l >> x)) != 0 {
                    return true;
                }
            }
        }
        false
    }

    fn place_on_field(&self, field: &mut Field, (x, y): (isize, isize)) {
        while field.rows.len() <= y as usize {
            field.rows.push(0);
        }

        for (py, l) in self.lines.iter().enumerate() {
            field.rows[y as usize - py] |= l >> x;
        }
    }
}

impl Game {
    #[allow(unused)]
    fn print(&self) {
        println!("Field {} {}", self.field.rows.len(), self.control.current);
        for c in (0..self.field.rows.len()).rev() {
            print!("#");
            for n in 0..7 {
                let v = ((self.field.rows[c] << n) & (1 << 6)) == (1 << 6);
                if v {
                    print!("*");
                } else {
                    print!(".");
                }
            }
            print!("#");
            println!();
        }
    }

    fn height(&self) -> usize {
        self.field.rows.len() + self.field.trimmed
    }

    fn add_pieces_bulk(&mut self, num: usize) {
        let mut hs = HashMap::new();

        // Loops until it finds a cycle where the game is repeating. This happens if the
        // trimmed field and instruction number are identical. Trimming a field removes everything
        // above a complete line, as pieces below the complete line cannot impact a falling piece

        loop {
            for _ in 0..5 {
                self.add_piece();
            }
            self.field.trim();

            // The clone doesn't seem intrinsically necessary. But it's running in ~1ms, so ¯\_(ツ)_/¯
            let existing = hs.get(&(self.field.rows.clone(), self.control.current));

            if let Some((tiles, piece_count)) = existing {
                let diff = self.piece_count - piece_count;
                let trimmed_diff = self.field.trimmed - tiles;

                let cycles = (num - self.piece_count) / diff;

                self.piece_count += cycles * diff;
                self.field.trimmed += cycles * trimmed_diff;
                break;
            } else {
                hs.insert(
                    (self.field.rows.clone(), self.control.current),
                    (self.field.trimmed, self.piece_count),
                );
            }
        }

        while self.piece_count < num {
            self.add_piece();
        }
    }

    fn add_pieces(&mut self, num: usize) {
        for _ in 0..num {
            self.add_piece();
        }
    }

    fn add_piece(&mut self) {
        let p = &self.pieces[self.piece_count % 5];
        self.piece_count += 1;

        let mut pos = (2, (self.field.rows.len() + 2 + p.lines.len()) as isize);

        loop {
            let dir = self.control.next();
            if !p.collides_with_field(&self.field, (pos.0 + dir, pos.1)) {
                pos.0 += dir;
            }

            if !p.collides_with_field(&self.field, (pos.0, pos.1 - 1)) {
                pos.1 -= 1;
            } else {
                break;
            }
        }

        p.place_on_field(&mut self.field, pos);
    }
}

pub fn day_17() -> (String, String) {
    let f = read_to_string("input/day17.txt").unwrap();
    let line = f.split("\n").nth(0).unwrap();

    let mut dirs = vec![];
    for n in line.chars() {
        if n == '<' {
            dirs.push(-1);
        } else {
            dirs.push(1);
        }
    }

    let mut game_a = Game {
        field: Field::default(),
        control: Control { dirs, current: 0 },
        piece_count: 0,
        pieces: Piece::default_pieces(),
    };

    let mut game_b = game_a.clone();

    game_a.add_pieces(2022);
    game_b.add_pieces_bulk(1_000_000_000_000);

    (
        format!("{}", game_a.height()),
        format!("{}", game_b.height()),
    )
}
