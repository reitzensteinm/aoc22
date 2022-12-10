use std::fs::read_to_string;

#[derive(Debug)]
struct Square {
    height: isize,
    visible: bool,
}

struct GridLine {
    squares: Vec<Square>,
}

struct Grid {
    lines: Vec<GridLine>,
}

impl Grid {
    fn width(&self) -> isize {
        self.lines[0].squares.len() as isize
    }
    fn height(&self) -> isize {
        self.lines.len() as isize
    }
    fn get(&mut self, x: isize, y: isize) -> Option<&mut Square> {
        if y < 0 || y >= self.lines.len() as isize {
            None
        } else {
            let line = &mut self.lines[y as usize];
            if x < 0 || x >= line.squares.len() as isize {
                None
            } else {
                Some(&mut line.squares[x as usize])
            }
        }
    }
}
pub fn day_8() -> (String, String) {
    let f = read_to_string("input/day8.txt").unwrap();

    let mut grid = Grid { lines: vec![] };

    for l in f.split("\n").filter(|l| l.len() > 0) {
        let mut squares = vec![];

        for c in l.chars() {
            let height = if c == '0' {
                0
            } else {
                ((c as isize) - ('1' as isize)) + 1
            };

            squares.push(Square {
                height,
                visible: false,
            });
        }

        grid.lines.push(GridLine { squares });
    }

    for y in 0..grid.lines.len() {
        {
            let mut highest = -1;
            for x in 0..grid.lines[y].squares.len() {
                let square = &mut grid.lines[y].squares[x];
                square.visible |= square.height > highest;
                highest = highest.max(square.height);
            }
        }

        {
            let mut highest = -1;
            for x in (0..grid.lines[y].squares.len()).rev() {
                let square = &mut grid.lines[y].squares[x];
                square.visible |= square.height > highest;
                highest = highest.max(square.height);
            }
        }
    }

    for x in 0..grid.lines[0].squares.len() {
        {
            let mut highest = -1;
            for y in 0..grid.lines.len() {
                let square = &mut grid.lines[y].squares[x];
                square.visible |= square.height > highest;
                highest = highest.max(square.height);
            }
        }

        {
            let mut highest = -1;
            for y in (0..grid.lines.len()).rev() {
                let square = &mut grid.lines[y].squares[x];
                square.visible |= square.height > highest;
                highest = highest.max(square.height);
            }
        }
    }

    let mut visible = 0;
    for y in 0..grid.lines.len() {
        for x in 0..grid.lines[y].squares.len() {
            if grid.lines[y].squares[x].visible {
                visible += 1;
            }
        }
    }

    let dirs = vec![[-1, 0], [1, 0], [0, -1], [0, 1]];

    let mut highest_score: Option<i32> = None;
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let height = grid.get(x, y).unwrap().height;

            let mut score = 1;
            for dir in &dirs {
                let mut pos = [x, y];
                let mut dist = 0;

                loop {
                    dist += 1;
                    pos[0] += dir[0];
                    pos[1] += dir[1];

                    if let Some(t) = grid.get(pos[0], pos[1]) {
                        if t.height >= height {
                            break;
                        }
                    } else {
                        dist -= 1;
                        break;
                    }
                }
                score *= dist;
            }

            if let Some(h) = highest_score {
                highest_score = Some(h.max(score));
            } else {
                highest_score = Some(score);
            }
        }
    }

    (
        format!("{}", visible),
        format!("{}", highest_score.unwrap()),
    )
}
