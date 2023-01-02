use std::fs::read_to_string;

// Dynamic Programming today, to change things up. My intuition is a tree search like A* may well
// end up faster - you're mostly going to be hammering down and to the right so should get an
// excellent outcome depth first. But I like this solution too.
// This one however would be pretty trivial to vectorize. You could fit the play field
// IN REGISTERS on an AVX-512 machine.

#[derive(Copy, Clone, PartialEq)]
enum Square {
    Empty,
    Wall,
    BlizzardNorth,
    BlizzardSouth,
    BlizzardEast,
    BlizzardWest,
}

struct Field {
    width: usize,
    height: usize,
    squares: Vec<Square>,
}

fn wrap(n: isize, div: isize) -> isize {
    let o = n % div;
    if o < 0 {
        div + o
    } else {
        o
    }
}

impl Field {
    fn wrap_blizzard(&self, (px, py): (usize, usize), (dx, dy): (isize, isize)) -> (usize, usize) {
        (
            (wrap((px as isize + dx) - 1, (self.width - 2) as isize) + 1) as usize,
            (wrap((py as isize + dy) - 1, (self.height - 2) as isize) + 1) as usize,
        )
    }

    fn get_square(&self, (px, py): (usize, usize)) -> Square {
        self.squares[py * self.width + px]
    }

    fn blocked(&self, pos: (usize, usize), ticks: usize) -> bool {
        if self.get_square(pos) == Square::Wall {
            return true;
        }

        if pos.0 >= 1 && pos.0 < self.width - 1 && pos.1 >= 1 && pos.1 < self.height - 1 {
            if self.get_square(self.wrap_blizzard(pos, (-(ticks as isize), 0)))
                == Square::BlizzardEast
            {
                return true;
            }

            if self.get_square(self.wrap_blizzard(pos, ((ticks as isize), 0)))
                == Square::BlizzardWest
            {
                return true;
            }

            if self.get_square(self.wrap_blizzard(pos, (0, -(ticks as isize))))
                == Square::BlizzardSouth
            {
                return true;
            }
            if self.get_square(self.wrap_blizzard(pos, (0, (ticks as isize))))
                == Square::BlizzardNorth
            {
                return true;
            }
        }

        false
    }
}

struct Pathfind {
    field: Field,
}

#[derive(Clone)]
struct PathfindState {
    ticks: usize,
    width: usize,
    height: usize,
    reachable: Vec<bool>,
}

impl PathfindState {
    fn iterate(&self, field: &Field) -> PathfindState {
        let mut pfs = self.clone();
        pfs.ticks += 1;

        for y in 0..self.height {
            for x in 0..self.width {
                let blocked = field.blocked((x, y), pfs.ticks);
                let mut neighbour = false;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx != 0 && dy != 0 {
                            continue;
                        }

                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx < 0
                            || ny < 0
                            || nx >= self.width as isize
                            || ny >= self.height as isize
                        {
                            continue;
                        }

                        if self.reachable[(ny * self.width as isize + nx) as usize] {
                            neighbour = true;
                        }
                    }
                }

                let reachable = if blocked { false } else { neighbour };

                pfs.reachable[y * self.width + x] = reachable;
            }
        }

        pfs
    }
}

impl Pathfind {
    fn search(&self, start: (usize, usize), end: (usize, usize), start_ticks: usize) -> usize {
        let mut state = PathfindState {
            ticks: start_ticks,
            width: self.field.width,
            height: self.field.height,
            reachable: vec![false; self.field.squares.len()],
        };

        state.reachable[start.1 * state.width + start.0] = true;

        for n in 1.. {
            state = state.iterate(&self.field);
            if state.reachable[end.1 * state.width + end.0] {
                return n;
            }
        }

        panic!()
    }
}

pub fn day_24() -> (String, String) {
    let f = read_to_string("input/day24.txt").unwrap();

    let file_lines = f.lines().collect::<Vec<&str>>();

    let width = file_lines[0].len();
    let height = file_lines.len();

    let mut squares = vec![];

    let mut start = None;
    let mut end = None;

    for (y, l) in file_lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            squares.push(match c {
                '#' => Square::Wall,
                '.' => {
                    if y == 0 {
                        start = Some((x, y));
                    } else if y == file_lines.len() - 1 {
                        end = Some((x, y));
                    }
                    Square::Empty
                }
                '>' => Square::BlizzardEast,
                '<' => Square::BlizzardWest,
                '^' => Square::BlizzardNorth,
                'v' => Square::BlizzardSouth,
                _ => panic!(),
            });
        }
    }

    let start = start.unwrap();
    let end = end.unwrap();

    let field = Field {
        width,
        height,
        squares,
    };

    let pathfind = Pathfind { field };

    let first = pathfind.search(start.clone(), end.clone(), 0);
    let second = pathfind.search(end.clone(), start.clone(), first);
    let third = pathfind.search(start.clone(), end.clone(), second + first);

    (format!("{}", first), format!("{}", first + second + third))
}
