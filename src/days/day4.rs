use regex::Regex;
use std::fs::read_to_string;

#[derive(Copy, Clone, Debug)]
struct Range {
    from: u32,
    to: u32,
}

impl Range {
    pub fn new(from: u32, to: u32) -> Range {
        Range { from, to }
    }
    pub fn contains(self, other: Range) -> bool {
        other.from >= self.from && other.to <= self.to
    }

    pub fn overlaps(self, other: Range) -> bool {
        let lower = other.from < self.from && other.to < self.from;
        let higher = other.from > self.to && other.to > self.to;

        !(lower || higher)
    }
}

pub fn day_4() -> (String, String) {
    let f = read_to_string("input/day4.txt").unwrap();

    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    let mut fully_contains = 0;
    let mut overlaps = 0;

    for l in f.lines() {
        let c = re.captures(l).unwrap();

        let a = Range::new(str::parse(&c[1]).unwrap(), str::parse(&c[2]).unwrap());
        let b = Range::new(str::parse(&c[3]).unwrap(), str::parse(&c[4]).unwrap());

        if a.contains(b) || b.contains(a) {
            fully_contains += 1;
        }

        if a.overlaps(b) {
            overlaps += 1;
        }
    }

    (format!("{}", fully_contains), format!("{}", overlaps))
}
