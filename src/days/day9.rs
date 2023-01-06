use std::collections::HashSet;
use std::fs::read_to_string;

struct RopeSim {
    knots: Vec<[isize; 2]>,
    visited: HashSet<[isize; 2]>,
}

impl RopeSim {
    fn move_head(&mut self, dir: [isize; 2], mag: usize) {
        for _ in 0..mag {
            self.knots[0][0] += dir[0];
            self.knots[0][1] += dir[1];

            for k in 0..self.knots.len() - 1 {
                let head = &self.knots[k];
                let tail = &self.knots[k + 1];

                let diffx = head[0] - tail[0];
                let diffy = head[1] - tail[1];

                if head[0] == tail[0] {
                    if diffy.abs() == 2 {
                        self.knots[k + 1][1] += diffy.clamp(-1, 1)
                    }
                } else if head[1] == tail[1] {
                    if diffx.abs() == 2 {
                        self.knots[k + 1][0] += diffx.clamp(-1, 1)
                    }
                } else if diffx.abs() == 2 || diffy.abs() == 2 {
                    self.knots[k + 1][0] += diffx.clamp(-1, 1);
                    self.knots[k + 1][1] += diffy.clamp(-1, 1);
                }
            }

            self.visited.insert(*self.knots.last().unwrap());
        }
    }
}

pub fn day_9() -> (String, String) {
    let f = read_to_string("input/day9.txt").unwrap();

    let mut sim = RopeSim {
        knots: vec![[0, 0], [0, 0]],
        visited: HashSet::new(),
    };

    let mut long_sim = RopeSim {
        knots: vec![[0, 0]; 10],
        visited: HashSet::new(),
    };

    long_sim.visited.insert([0, 0]);
    sim.visited.insert([0, 0]);

    for l in f.lines() {
        let (dir_str, mag_str) = l.split_once(" ").unwrap();

        // I hope this is code motioned out...
        let dir = if dir_str == "U" {
            [0, -1]
        } else if dir_str == "D" {
            [0, 1]
        } else if dir_str == "L" {
            [-1, 0]
        } else if dir_str == "R" {
            [1, 0]
        } else {
            todo!()
        };

        let mag = str::parse::<usize>(mag_str).unwrap();

        sim.move_head(dir, mag);
        long_sim.move_head(dir, mag);
    }

    (
        format!("{}", sim.visited.len()),
        format!("{}", long_sim.visited.len()),
    )
}
