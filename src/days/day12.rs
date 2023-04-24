use std::fs::read_to_string;

type Position = (isize, isize);

#[derive(Copy, Clone)]
struct Square {
    height: u8,
    visited: bool,
    cost: usize,
}

struct Grid {
    reverse: bool,
    squares: Vec<Vec<Square>>,
    start: Position,
    end: Position,
}

struct PathfindParital {
    positions: Vec<Position>,
}

enum PathfindStage {
    Partial(PathfindParital),
    Complete(usize),
}

impl Grid {
    fn pathfind(&mut self) -> usize {
        let mut ps = PathfindStage::Partial(PathfindParital {
            positions: vec![self.start],
        });
        loop {
            match ps {
                PathfindStage::Partial(p) => {
                    ps = self.iterate(p);
                }
                PathfindStage::Complete(v) => {
                    return v;
                }
            }
        }
    }

    fn get(&mut self, p: Position) -> Option<&mut Square> {
        let (px, py) = p;
        if py >= 0 && py < self.squares.len() as isize {
            let l = &mut self.squares[py as usize];
            if px >= 0 && px < l.len() as isize {
                return Some(&mut l[px as usize]);
            }
        }
        None
    }

    fn iterate(&mut self, ps: PathfindParital) -> PathfindStage {
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let mut next = vec![];
        for (px, py) in ps.positions {
            let current = *self.get((px, py)).unwrap();
            let end = self.end;

            // The "reverse" here makes me sad. A boxed callback in an inner loops is too slow
            // It's surely possible to abstract this cleanly and cheaply, my rust isn't there yet
            let reverse = self.reverse;
            for (dx, dy) in directions {
                if let Some(sq) = self.get((px + dx, py + dy)) {
                    let height_check = if reverse {
                        sq.height >= current.height - 1
                    } else {
                        sq.height <= current.height + 1
                    };

                    if height_check && !sq.visited {
                        sq.visited = true;
                        sq.cost = current.cost + 1;

                        if reverse {
                            if sq.height == 0 {
                                return PathfindStage::Complete(sq.cost);
                            }
                        } else if (px + dx, py + dy) == end {
                            return PathfindStage::Complete(sq.cost);
                        }

                        next.push((px + dx, py + dy));
                    }
                }
            }
        }

        PathfindStage::Partial(PathfindParital { positions: next })
    }
}

pub fn day_12() -> (String, String) {
    let f = read_to_string("input/day12.txt").unwrap();

    let mut start = None;
    let mut end = None;

    let mut lines = vec![];
    for (cy, l) in f.lines().enumerate() {
        let mut line = vec![];
        for (cx, c) in l.chars().enumerate() {
            let height = if c == 'S' {
                start = Some((cx as isize, cy as isize));
                0
            } else if c == 'E' {
                end = Some((cx as isize, cy as isize));
                25
            } else {
                (c as u8) - b'a'
            };

            let sq = Square {
                height,
                cost: 0,
                visited: false,
            };
            line.push(sq);
        }
        lines.push(line);
    }

    let mut grid_a = Grid {
        squares: lines.clone(),
        reverse: false,
        start: start.unwrap(),
        end: end.unwrap(),
    };

    let mut grid_b = Grid {
        squares: lines,
        reverse: true,
        start: end.unwrap(),
        end: end.unwrap(),
    };

    (
        format!("{}", grid_a.pathfind()),
        format!("{}", grid_b.pathfind()),
    )
}
