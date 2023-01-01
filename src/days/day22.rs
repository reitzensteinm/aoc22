use regex::Regex;
use std::fs::read_to_string;

#[derive(Copy, Clone)]
struct State {
    x: isize,
    y: isize,
    rotation: isize,
}

impl State {
    fn rotate(&mut self, dir: isize) {
        self.rotation += dir;
        if self.rotation < 0 {
            self.rotation = 3;
        } else if self.rotation > 3 {
            self.rotation = 0;
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Piece {
    OffMap,
    Open,
    Blocked,
}

#[derive(Copy, Clone)]
enum Instruction {
    Left,
    Right,
    Move(usize),
}

struct Board {
    cube: bool,
    lines: Vec<Vec<Piece>>,
}

impl Board {
    fn next_piece_cube(&self, state: State) -> State {
        // Not particularly happy with this code - there must be a way to generalize
        // But it's pretty fast, so...
        let (dx, dy) = [(1, 0), (0, 1), (-1, 0), (0, -1)][state.rotation as usize];

        let mut next_state = State {
            x: state.x + dx,
            y: state.y + dy,
            rotation: state.rotation,
        };

        let sq = |x, y| ((x + 50) / 50, (y + 50) / 50);

        let curs = sq(state.x, state.y);
        let news = sq(next_state.x, next_state.y);

        let to_left = |i, b: (isize, isize)| State {
            x: (b.0 - 1) * 50,
            y: (b.1 - 1) * 50 + i,
            rotation: 0,
        };

        let to_right = |i, b: (isize, isize)| State {
            x: (b.0 - 1) * 50 + 49,
            y: (b.1 - 1) * 50 + i,
            rotation: 2,
        };

        let to_up = |i, b: (isize, isize)| State {
            x: (b.0 - 1) * 50 + i,
            y: (b.1 - 1) * 50,
            rotation: 1,
        };

        let to_down = |i, b: (isize, isize)| State {
            x: (b.0 - 1) * 50 + i,
            y: (b.1 - 1) * 50 + 49,
            rotation: 3,
        };

        if curs != news {
            let x = state.x % 50;
            let y = state.y % 50;
            let y_flip = 49 - y;

            next_state = match (curs, news) {
                // These were literally made by cutting out the cube, placing it in 3D,
                // and figuring out where everything went. This layout is valid only for
                // my input's layout. I really wish I could figure out a way to cleanly
                // generalize, but I'm ready to move on.
                ((2, 1), (3, 1)) => next_state,
                ((2, 1), (2, 2)) => next_state,
                ((2, 1), (2, 0)) => to_left(x, (1, 4)),
                ((2, 1), (1, 1)) => to_left(y_flip, (1, 3)),

                ((1, 3), (1, 4)) => next_state,
                ((1, 3), (2, 3)) => next_state,
                ((1, 3), (0, 3)) => to_left(y_flip, (2, 1)),
                ((1, 3), (1, 2)) => to_left(x, (2, 2)),

                ((1, 4), (1, 3)) => next_state,
                ((1, 4), (0, 4)) => to_up(y, (2, 1)),
                ((1, 4), (1, 5)) => to_up(x, (3, 1)),
                ((1, 4), (2, 4)) => to_down(y, (2, 3)),

                ((2, 3), (1, 3)) => next_state,
                ((2, 3), (2, 2)) => next_state,
                ((2, 3), (2, 4)) => to_right(x, (1, 4)),
                ((2, 3), (3, 3)) => to_right(y_flip, (3, 1)),

                ((2, 2), (2, 1)) => next_state,
                ((2, 2), (2, 3)) => next_state,
                ((2, 2), (3, 2)) => to_down(y, (3, 1)),
                ((2, 2), (1, 2)) => to_up(y, (1, 3)),

                ((3, 1), (2, 1)) => next_state,
                ((3, 1), (4, 1)) => to_right(y_flip, (2, 3)),
                ((3, 1), (3, 0)) => to_down(x, (1, 4)),
                ((3, 1), (3, 2)) => to_right(x, (2, 2)),

                _ => todo!(),
            };
        }

        next_state
    }

    fn next_piece(&self, state: State) -> State {
        let (mut px, mut py) = (state.x, state.y);

        let (dx, dy) = [(1, 0), (0, 1), (-1, 0), (0, -1)][state.rotation as usize];

        loop {
            px += dx;
            py += dy;

            if px < 0 {
                px = (self.lines[0].len() - 1) as isize;
            } else if px >= self.lines[0].len() as isize {
                px = 0;
            }

            if py < 0 {
                py = (self.lines.len() - 1) as isize;
            } else if py >= self.lines.len() as isize {
                py = 0;
            }

            if self.lines[py as usize][px as usize] != Piece::OffMap {
                return State {
                    x: px,
                    y: py,
                    rotation: state.rotation,
                };
            }
        }
    }

    fn move_piece(&mut self, mut state: State, len: usize) -> State {
        for _ in 0..len {
            let next = if self.cube {
                self.next_piece_cube(state)
            } else {
                self.next_piece(state)
            };

            if self.lines[next.y as usize][next.x as usize] == Piece::Blocked {
                return state;
            } else {
                state = next;
            }
        }

        state
    }

    fn read(lines: &[&str]) -> Board {
        let len = lines.iter().map(|v| v.len()).max().unwrap();
        let mut board = Board {
            lines: vec![],
            cube: false,
        };

        for l in lines {
            let chars = l.chars().collect::<Vec<char>>();
            let mut board_line = vec![];

            for n in 0..len {
                board_line.push(match *chars.get(n).unwrap_or(&' ') {
                    ' ' => Piece::OffMap,
                    '.' => Piece::Open,
                    '#' => Piece::Blocked,
                    _ => panic!(),
                })
            }

            board.lines.push(board_line);
        }

        board
    }
}

struct Game {
    board: Board,
    instructions: Vec<Instruction>,
    state: State,
}

impl Game {
    fn new(board: Board, instructions: Vec<Instruction>) -> Game {
        Game {
            board,
            instructions,
            state: State {
                x: 0,
                y: 0,
                rotation: 0,
            },
        }
    }

    fn reset(&mut self, cube: bool) {
        self.board.cube = cube;
        for (x, piece) in self.board.lines[0].iter().enumerate() {
            if matches!(piece, Piece::Open) {
                self.state.x = x as isize;
                self.state.y = 0;
                self.state.rotation = 0;
                return;
            }
        }
        panic!();
    }

    fn run(&mut self) -> isize {
        for i in &self.instructions {
            match i {
                Instruction::Left => {
                    self.state.rotate(-1);
                }
                Instruction::Right => {
                    self.state.rotate(1);
                }
                Instruction::Move(length) => {
                    self.state = self.board.move_piece(self.state, *length);
                }
            }
        }

        (self.state.y + 1) * 1000 + (self.state.x + 1) * 4 + self.state.rotation as isize
    }
}

pub fn day_22() -> (String, String) {
    let f = read_to_string("input/day22.txt").unwrap();

    let lines = f.lines().collect::<Vec<&str>>();

    let board = Board::read(&lines[0..lines.len() - 2]);
    let path = lines[lines.len() - 1];

    let re = Regex::new(r"(\d+|R|L)").unwrap();
    let mut instructions = vec![];

    for c in re.captures_iter(path) {
        instructions.push(if &c[0] == "L" {
            Instruction::Left
        } else if &c[0] == "R" {
            Instruction::Right
        } else {
            Instruction::Move(str::parse::<usize>(&c[0]).unwrap())
        })
    }

    let mut g = Game::new(board, instructions);

    g.reset(false);
    let part_a = g.run();
    g.reset(true);
    let part_b = g.run();

    (format!("{part_a}"), format!("{part_b}"))
}
