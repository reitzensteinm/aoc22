use std::fs::read_to_string;

#[derive(Clone)]
enum MonkeyOp {
    Square,
    Add(usize),
    Multiply(usize),
}

#[derive(Clone)]
struct Item {
    owner: usize,
    worry: usize,
}

#[derive(Clone)]
struct Monkey {
    inspected_count: usize,
    num: usize,
    op: MonkeyOp,
    route: [usize; 3],
}

impl Monkey {
    fn run(&mut self, divisor: usize, co_prime: usize, items: &mut [Item]) {
        for m in items {
            if m.owner == self.num {
                self.inspected_count += 1;

                let mut new_level = match self.op {
                    MonkeyOp::Square => m.worry * m.worry,
                    MonkeyOp::Add(n) => m.worry + n,
                    MonkeyOp::Multiply(n) => m.worry * n,
                };

                if divisor != 1 {
                    new_level = (new_level / divisor)
                }

                if new_level > 1_000_000_000 {
                    new_level = new_level % co_prime;
                }

                let target = if new_level % self.route[0] == 0 {
                    self.route[1]
                } else {
                    self.route[2]
                };

                m.worry = new_level;
                m.owner = target;
            }
        }
    }
}

struct MonkeyTroop {
    divisor: usize,
    co_prime: usize,
    items: Vec<Item>,
    monkeys: Vec<Monkey>,
}

impl MonkeyTroop {
    fn run(&mut self) {
        for n in 0..self.monkeys.len() {
            self.monkeys[n].run(self.divisor, self.co_prime, &mut self.items);
        }
    }

    fn monkey_business(&self) -> usize {
        let mut inspection_counts: Vec<usize> =
            self.monkeys.iter().map(|m| m.inspected_count).collect();

        inspection_counts.sort();
        inspection_counts.reverse();

        inspection_counts[0] * inspection_counts[1]
    }
}

pub fn day_11() -> (String, String) {
    let f = read_to_string("input/day11.txt").unwrap();

    let lines: Vec<&str> = f.split("\n").filter(|l| l.len() > 0).collect();

    let mut monkeys = vec![];
    let mut co_prime = 1;

    let mut items = vec![];

    for (m, x) in lines.chunks(6).enumerate() {
        let parse = |s| str::parse::<usize>(s).unwrap();

        let (_, items_str) = x[1].split_at("  Starting Items: ".len());

        for i in items_str.split(", ") {
            items.push(Item {
                owner: m,
                worry: parse(i),
            });
        }

        let (_, op_str) = x[2].split_at("  Operation: new = ".len());

        let op: MonkeyOp = if op_str == "old * old" {
            MonkeyOp::Square
        } else {
            let (_, o_str) = op_str.split_at("old _ ".len());
            let o = parse(o_str);
            if op_str.starts_with("old + ") {
                MonkeyOp::Add(o)
            } else {
                MonkeyOp::Multiply(o)
            }
        };

        let (_, test_str) = x[3].split_at("  Test: divisible by ".len());
        let (_, route_a) = x[4].split_at("    If true: throw to monkey ".len());
        let (_, route_b) = x[5].split_at("    If false: throw to monkey ".len());

        let route = [parse(test_str), parse(route_a), parse(route_b)];

        co_prime *= route[0];
        monkeys.push(Monkey {
            route,
            num: m,
            op,
            inspected_count: 0,
        });
    }

    let mut troop_1 = MonkeyTroop {
        monkeys: monkeys.clone(),
        items: items.clone(),
        co_prime,
        divisor: 1,
    };

    let mut troop_3 = MonkeyTroop {
        monkeys,
        items,
        co_prime,
        divisor: 3,
    };

    for _ in 0..20 {
        troop_3.run();
    }

    for _ in 0..10_000 {
        troop_1.run();
    }

    (
        format!("{}", troop_3.monkey_business()),
        format!("{}", troop_1.monkey_business()),
    )
}
