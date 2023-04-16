use crate::utils::bit_grid::{BitGrid, BitView, Or, Prim, Shifted, ShiftedWrap, Window};
use std::fs::read_to_string;

pub fn day_24() -> (String, String) {
    let f = read_to_string("input/day24.txt").unwrap();

    let file_lines = f.lines().collect::<Vec<&str>>();

    let width = file_lines[0].len();
    let height = file_lines.len();

    let mut start = None;
    let mut end = None;

    let mut wall = BitGrid::new(width, height);

    // N S E W
    let mut blizzards = vec![BitGrid::new(width, height); 4];

    for (y, l) in file_lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '#' => wall.set(x, y, true),
                '.' => {
                    if y == 0 {
                        start = Some((x, y));
                    } else if y == file_lines.len() - 1 {
                        end = Some((x, y));
                    }
                }

                '^' => blizzards[0].set(x, y, true),
                'v' => blizzards[1].set(x, y, true),
                '>' => blizzards[2].set(x, y, true),
                '<' => blizzards[3].set(x, y, true),
                _ => {}
            }
        }
    }

    let width = wall.width() as isize;
    let height = wall.height() as isize;

    let pathfind = |from: (usize, usize), to: (usize, usize), time_off: isize| -> isize {
        let mut time = time_off;
        let mut locations = BitGrid::new(width as usize, height as usize);
        locations.set(from.0, from.1, true);
        loop {
            let sa = Shifted::new(&locations, 1, 0);
            let sb = Shifted::new(&locations, -1, 0);
            let sc = Shifted::new(&locations, 0, -1);
            let sd = Shifted::new(&locations, 0, 1);

            let acc = Or::new(&locations, &sa);
            let acc = Or::new(&acc, &sb);
            let acc = Or::new(&acc, &sc);
            let acc = Or::new(&acc, &sd);

            let mut moved_blizzards = vec![];
            for x in 0..4 {
                let dir = vec![(0, -time), (0, time), (time, 0), (-time, 0)][x];
                let w = Window::new(&blizzards[x], 1, 1, width - 2, height - 2);
                let s = ShiftedWrap::new(&w, dir.0, dir.1);
                let nw = Window::new(&s, 0, 0, width, height);
                let out = Shifted::new(&nw, 1, 1);
                moved_blizzards.push(BitGrid::from_view(&out));
            }

            let acc = Prim::new(&acc, &moved_blizzards[0], |a, b| a & (!b));
            let acc = Prim::new(&acc, &moved_blizzards[1], |a, b| a & (!b));
            let acc = Prim::new(&acc, &moved_blizzards[2], |a, b| a & (!b));
            let acc = Prim::new(&acc, &moved_blizzards[3], |a, b| a & (!b));
            let acc = Prim::new(&acc, &wall, |a, b| a & (!b));

            locations = BitGrid::from_view(&acc);

            if locations.get(to.0, to.1) {
                return time;
            }
            time += 1;
        }
    };

    let a = pathfind(start.unwrap(), end.unwrap(), 0);
    let b = pathfind(end.unwrap(), start.unwrap(), a);
    let c = pathfind(start.unwrap(), end.unwrap(), b);

    (format!("{}", a), format!("{}", c))
}
