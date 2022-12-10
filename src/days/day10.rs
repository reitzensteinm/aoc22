use std::fs::read_to_string;

struct Machine {
    reg: isize,
    pointer: usize,
    cycle_number: usize,
    pixels: Vec<bool>,
    instructions: Vec<isize>,
}

impl Machine {
    fn advance(&mut self) {
        let cn = ((self.cycle_number - 1) % 40) as isize;

        if cn == self.reg || cn == self.reg - 1 || cn == self.reg + 1 {
            self.pixels[self.cycle_number] = true;
        }

        self.reg += self.instructions[self.pointer];
        self.pointer += 1;
        self.cycle_number += 1;
        if self.pointer > self.instructions.len() {
            self.pointer = 0;
        }
    }

    fn bulk_advance(&mut self, n: usize) -> isize {
        let mut p = 0;
        for _ in 0..n {
            p = self.reg * (self.cycle_number as isize);
            self.advance();
        }
        p
    }
}

pub fn day_10() -> (String, String) {
    let f = read_to_string("input/day10.txt").unwrap();

    let mut m = Machine {
        reg: 1,
        pointer: 0,
        pixels: vec![false; 240],
        cycle_number: 1,
        instructions: vec![],
    };

    for l in f.split("\n").filter(|l| l.len() > 0) {
        m.instructions.push(0);

        if l != "noop" {
            let (_, v) = l.split_once(" ").unwrap();
            let off = str::parse::<isize>(v).unwrap();
            m.instructions.push(off);
        };
    }

    let mut sum = 0;
    sum += m.bulk_advance(20);
    for _ in 0..5 {
        sum += m.bulk_advance(40);
    }

    m.bulk_advance(20);

    #[cfg(test)]
    for chunk in m.pixels.chunks(40) {
        for x in chunk {
            print!("{}", if *x { "#" } else { "." });
        }
        println!();
    }

    (format!("{}", sum), "read ascii".to_string())
}
