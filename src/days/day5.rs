use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
enum Mode {
    Single,
    Multi,
}

#[derive(Debug)]
struct Floor {
    mode: Mode,
    stacks: Vec<Vec<char>>,
}

impl Floor {
    fn transfer(&mut self, from: usize, to: usize, amount: usize) {
        let mut inter = vec![];
        for _ in 0..amount {
            inter.push(self.stacks[from].pop().unwrap());
        }

        if self.mode == Mode::Multi {
            // Not particularly fast, but unlikely to matter
            inter.reverse();
        }

        self.stacks[to].append(&mut inter);
    }

    fn describe(&self) -> String {
        let mut out = String::new();
        for s in &self.stacks {
            out.push(*(s.last().unwrap()));
        }
        out
    }
}

pub fn day_5() -> (String, String) {
    let f = read_to_string("input/day5.txt").unwrap();

    let mut lines = f.lines();

    let stack_lines: Vec<Vec<char>> = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let mut floor_single = Floor {
        stacks: vec![],
        mode: Mode::Single,
    };
    let mut floor_multi = Floor {
        stacks: vec![],
        mode: Mode::Multi,
    };

    for x in 0..9 {
        let mut stack = vec![];
        for y in 0..stack_lines.len() - 1 {
            let char = stack_lines[stack_lines.len() - (y + 2)][x * 4 + 1];
            if char != ' ' {
                stack.push(char);
            }
        }
        floor_single.stacks.push(stack.clone());
        floor_multi.stacks.push(stack);
    }

    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for instr in lines.take_while(|l| !l.is_empty()) {
        let c = re.captures(instr).unwrap();
        let from = str::parse::<usize>(&c[2]).unwrap() - 1;
        let to = str::parse::<usize>(&c[3]).unwrap() - 1;
        let amount = str::parse::<usize>(&c[1]).unwrap();

        floor_single.transfer(from, to, amount);
        floor_multi.transfer(from, to, amount);
    }

    (floor_single.describe(), floor_multi.describe())
}
