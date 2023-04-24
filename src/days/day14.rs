use std::fs::read_to_string;

type Pos = (isize, isize);

const MAP_HEIGHT: usize = 200;
const MAP_WIDTH: usize = 400;
const MAP_OFFSET: usize = 300;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Square {
    Rock,
    Air,
    Sand,
}

#[derive(Clone)]
struct Grid {
    cells: Vec<Vec<Square>>,
    lowest_y: isize,
    sand_count: usize,
}

impl Grid {
    fn get(&mut self, pos: Pos) -> &mut Square {
        let (px, py) = pos;
        &mut self.cells[py as usize][px as usize - MAP_OFFSET]
    }

    fn paint(&mut self, from: Pos, to: Pos) {
        let (from_x, from_y) = from;
        let (to_x, to_y) = to;

        if from_x == to_x {
            for y in from_y.min(to_y)..=from_y.max(to_y) {
                self.lowest_y = self.lowest_y.max(y);
                *self.get((from_x, y)) = Square::Rock;
            }
        } else {
            for x in from_x.min(to_x)..=from_x.max(to_x) {
                self.lowest_y = self.lowest_y.max(from_y);
                *self.get((x, from_y)) = Square::Rock;
            }
        }
    }

    fn spawn_sand(&mut self, pos: Pos) -> bool {
        let (px, py) = pos;

        if py >= (MAP_HEIGHT - 1) as isize {
            return true;
        }

        for x in [0, -1, 1] {
            let p = (px + x, py + 1);
            let g = self.get(p);

            if *g == Square::Air && self.spawn_sand(p) {
                return true;
            }
        }

        *self.get(pos) = Square::Sand;
        self.sand_count += 1;

        false
    }

    #[allow(unused)]
    fn show(&mut self) {
        for y in 0..self.cells.len() {
            println!();

            for x in 0..self.cells[y].len() {
                print!(
                    "{}",
                    if self.cells[y][x] == Square::Air {
                        " "
                    } else {
                        "R"
                    }
                );
            }
        }
    }
}

pub fn day_14() -> (String, String) {
    let f = read_to_string("input/day14.txt").unwrap();

    let mut out = vec![];
    for _ in 0..MAP_HEIGHT {
        out.push(vec![Square::Air; MAP_WIDTH]);
    }

    let mut grid = Grid {
        cells: out,
        lowest_y: 0,
        sand_count: 0,
    };

    for l in f.lines() {
        let locations = l.split(" -> ");

        let mut last = None;
        for loc in locations {
            let (xs, ys) = loc.split_once(',').unwrap();
            let x = str::parse::<isize>(xs).unwrap();
            let y = str::parse::<isize>(ys).unwrap();

            if let Some((old_x, old_y)) = last {
                grid.paint((old_x, old_y), (x, y));
            }

            last = Some((x, y));
        }
    }

    let mut grid_floor = grid.clone();

    grid_floor.paint(
        (MAP_OFFSET as isize, grid_floor.lowest_y + 2),
        (
            (MAP_OFFSET + MAP_WIDTH - 1) as isize,
            grid_floor.lowest_y + 2,
        ),
    );

    grid_floor.spawn_sand((500, 0));
    grid.spawn_sand((500, 0));

    (
        format!("{}", grid.sand_count),
        format!("{}", grid_floor.sand_count),
    )
}
