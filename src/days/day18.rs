use std::fs::read_to_string;

const OFFSETS: [(isize, isize, isize); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, 0, -1),
    (0, 0, 1),
    (0, -1, 0),
    (0, 1, 0),
];

#[derive(Copy, Clone)]
struct Point {
    solid: bool,
    external: bool,
}

impl Point {
    fn new() -> Point {
        Point {
            solid: false,
            external: false,
        }
    }
}

struct Grid {
    size: isize,
    offset: isize,
    fields: Vec<Point>,
}

impl Grid {
    fn new(size: isize) -> Grid {
        Grid {
            fields: vec![Point::new(); (size * size * size) as usize],
            offset: 2,
            size,
        }
    }

    fn get(&mut self, point: (isize, isize, isize)) -> Option<&mut Point> {
        if point.0 + self.offset < 0
            || point.0 >= self.size - self.offset
            || point.1 + self.offset < 0
            || point.1 >= self.size - self.offset
            || point.2 + self.offset < 0
            || point.2 >= self.size - self.offset
        {
            return None;
        }

        Some(
            &mut self.fields[((point.0 + self.offset) * self.size * self.size
                + (point.1 + self.offset) * self.size
                + (point.2 + self.offset)) as usize],
        )
    }
}

fn flood_fill_external(grid: &mut Grid, p: (isize, isize, isize)) {
    let mut points = vec![p];

    grid.get(p).unwrap().external = true;

    loop {
        let mut next = vec![];
        for p in points {
            for (ox, oy, oz) in &OFFSETS {
                let np = (ox + p.0, oy + p.1, oz + p.2);
                if let Some(cube) = grid.get(np) {
                    if !(cube.solid || cube.external) {
                        cube.external = true;
                        next.push(np)
                    }
                }
            }
        }

        if next.is_empty() {
            return;
        }
        points = next;
    }
}

pub fn day_18() -> (String, String) {
    let f = read_to_string("input/day18.txt").unwrap();

    let mut points = vec![];

    for l in f.lines() {
        let v = l.split(',').collect::<Vec<&str>>();
        let parse = |i| str::parse::<isize>(*v.get(i).unwrap()).unwrap();
        let point = (parse(0), parse(1), parse(2));
        points.push(point)
    }

    let mut grid = Grid::new(26);

    for p in &points {
        grid.get(*p).unwrap().solid = true;
    }

    flood_fill_external(&mut grid, (0, 0, 0));

    let mut surface = 0;
    let mut surface_external = 0;

    for (px, py, pz) in &points {
        for (ox, oy, oz) in &OFFSETS {
            let cube = grid.get((px + ox, py + oy, pz + oz)).unwrap();
            if !cube.solid {
                surface += 1;
                if cube.external {
                    surface_external += 1;
                }
            }
        }
    }

    (format!("{surface}"), format!("{surface_external}"))
}
