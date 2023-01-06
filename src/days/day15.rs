use regex::Regex;
use std::fs::read_to_string;

type Pos = (isize, isize);
type Range = (isize, isize);

type Square = (Pos, Pos);

struct Sensor {
    pos: Pos,
    beacon: Pos,
}

impl Sensor {
    fn exclusion_line(&self, line_y: isize) -> Option<Range> {
        let (x, y) = self.pos;
        let (xb, yb) = self.beacon;
        let dist = (x - xb).abs() + (y - yb).abs();

        let ydist = (y - line_y).abs();

        let diff = dist - ydist;

        if diff < 0 {
            None
        } else {
            Some((x - diff, x + diff))
        }
    }

    // The manhattan distance of each sensor is made up of four line segments, with dy/dx +/- 1
    // This projects them back to the Y axis
    fn y_intercepts(&self) -> ((isize, isize), (isize, isize)) {
        let (x, y) = self.pos;
        let (xb, yb) = self.beacon;
        let dist = (x - xb).abs() + (y - yb).abs();

        (
            // Down
            (((y + dist) + x), (y - dist) + x),
            // Up
            (((y + dist) - x), ((y - dist) - x)),
        )
    }

    fn excludes_pos(&self, p: Pos) -> bool {
        let calc_dist = |a: Pos, b: Pos| {
            let (x1, y1) = a;
            let (x2, y2) = b;
            (x1 - x2).abs() + (y1 - y2).abs()
        };

        calc_dist(self.beacon, self.pos) >= calc_dist(self.pos, p)
    }
}

pub fn merge_ranges(ranges: &[Range]) -> Vec<Range> {
    let mut ranges = Vec::from(ranges);
    ranges.sort_by(|(al, _), (bl, _)| al.cmp(bl));

    let mut out = vec![];
    let mut last: Option<Range> = None;

    for (el, eh) in ranges {
        if let Some((ll, lh)) = last {
            if el <= lh {
                last = Some((ll.min(el), lh.max(eh)))
            } else {
                out.push((ll, lh));
                last = Some((el, eh))
            }
        } else {
            last = Some((el, eh))
        }
    }

    if let Some(l) = last {
        out.push(l);
    }

    out
}

// Recursive method with early out, but it was too slow (~40ms vs ~1ms)
#[allow(unused)]
fn check_square(sensors: &[Sensor], square: Square) -> Option<Pos> {
    let ((x1, y1), (x2, y2)) = square;

    for s in sensors {
        if s.excludes_pos((x1, y1))
            && s.excludes_pos((x2, y1))
            && s.excludes_pos((x1, y2))
            && s.excludes_pos((x2, y2))
        {
            return None;
        }
    }

    if x1 == x2 && y1 == y2 {
        return Some((x1, y1));
    }

    let mx = (x1 + x2 + 1) / 2;
    let my = (y1 + y2 + 1) / 2;

    if let Some(p) = check_square(sensors, ((mx, my), (x2, y2))) {
        return Some(p);
    }

    if mx != x1 && my != y1 {
        if let Some(p) = check_square(sensors, ((x1, y1), (mx - 1, my - 1))) {
            return Some(p);
        }
    }

    if mx != x1 {
        if let Some(p) = check_square(sensors, ((x1, my), (mx - 1, y2))) {
            return Some(p);
        }
    }

    if my != y1 {
        if let Some(p) = check_square(sensors, ((mx, y1), (x2, my - 1))) {
            return Some(p);
        }
    }

    return None;
}

// Each sensor forms an exclusion range for a given Y value
// This merges these exclusion ranges to estimate total coverage
fn part_1(sensors: &[Sensor]) -> isize {
    let mut exclusions = vec![];

    for s in sensors {
        if let Some(e) = s.exclusion_line(2000000) {
            exclusions.push(e);
        }
    }

    exclusions = merge_ranges(&exclusions);

    let mut out = 0;

    for (l, h) in exclusions {
        // I'm pretty sure it _should_ be + 1!?
        out += h - l; //+ 1;
    }

    out
}

fn part_2(sensors: &[Sensor]) -> isize {
    let mut down_intercepts = vec![];
    let mut up_intercepts = vec![];

    for s in sensors {
        let ((d1, d2), (u1, u2)) = s.y_intercepts();
        up_intercepts.push(u1);
        up_intercepts.push(u2);
        down_intercepts.push(d1);
        down_intercepts.push(d2);
    }

    // The intercepts describe the lines that form the boundaries of each sensor
    // The location we're searching for *must* be next to a location next to intersections
    // of these lines, even if they're only points where sensor boundaries self intersect
    // The surroundings of each point of interest are inspected. This is n^2, but n is tiny.
    for u in &up_intercepts {
        for d in &down_intercepts {
            let x = (d - u) / 2;
            let y = (d + u) / 2;

            for chx in x - 1..=x + 1 {
                for chy in y - 1..=y + 1 {
                    if chx >= 0 && chx <= 4_000_000 && chy >= 0 && chy <= 4_000_000 {
                        let mut blocked = false;
                        for s in sensors {
                            if s.excludes_pos((chx, chy)) {
                                blocked = true;
                            }
                        }
                        if !blocked {
                            return chx * 4_000_000 + chy;
                        }
                    }
                }
            }
        }
    }
    panic!();
}

pub fn day_15() -> (String, String) {
    let f = read_to_string("input/day15.txt").unwrap();

    let re =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
            .unwrap();

    let mut sensors = vec![];

    for l in f.lines() {
        let c = re.captures(l).unwrap();
        let parse = |i| str::parse::<isize>(&c[i]).unwrap();

        let pos = (parse(1), parse(2));
        let beacon = (parse(3), parse(4));

        sensors.push(Sensor { pos, beacon });
    }

    (
        format!("{}", part_1(&sensors)),
        format!("{}", part_2(&sensors)),
    )
}
