use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time;

#[derive(Debug)]
struct Valve {
    num: usize,
    flow: usize,
    connections: Vec<usize>,
    times: Vec<usize>,
}

struct System {
    valves: Vec<Valve>,
    flow_priority: Vec<usize>,
}

impl System {
    fn pathfind(&mut self) {
        for i in 0..self.valves.len() {
            let mut times: Vec<Option<usize>> = vec![None; self.valves.len()];
            fn pf(s: &System, times: &mut Vec<Option<usize>>, p: usize, d: usize) {
                if let Some(t) = times[p] {
                    if t > d {
                        times[p] = None;
                    }
                }

                if times[p].is_none() {
                    times[p] = Some(d);

                    for x in &s.valves[p].connections {
                        pf(s, times, *x, d + 1);
                    }
                }
            }
            pf(&self, &mut times, self.valves[i].num, 0);

            self.valves[i].times = times.iter().map(|n| n.unwrap()).collect();
        }
    }

    fn priority(&mut self) {
        let priorities: Vec<usize> = self
            .valves
            .iter()
            .filter(|v| v.flow > 0)
            .sorted_by(|va, vb| vb.flow.cmp(&va.flow))
            .map(|v| v.num)
            .collect();

        self.flow_priority = priorities;
    }

    fn precalc(&mut self) {
        self.pathfind();
        self.priority();
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, PartialOrd, Ord)]
struct Entity {
    pos: u8,
    dest: Option<(u8, u8)>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord)]
struct SearchState {
    a: Entity,
    b: Entity,
    enabled: u64,
    volume: u32,
    step: u8,
    reward: usize,
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.step
                .cmp(&other.step)
                .then(self.reward.cmp(&other.reward)),
        )
    }
}

impl SearchState {
    fn calculate_reward(&mut self, search: &Search) {
        let mut reward = self.total_flow(search) as usize;
        if let Some((score, dist)) = self.a.dest {
            reward += score as usize / dist as usize;
        }

        if let Some((score, dist)) = self.b.dest {
            reward += score as usize / dist as usize;
        }
        self.reward = reward;
    }

    fn total_flow(&self, search: &Search) -> u32 {
        let mut out = 0;
        for x in &search.system.flow_priority {
            if (self.enabled >> *x) & 1 == 1 {
                let valve = &search.system.valves[*x];
                out += valve.flow;
            }
        }
        out as u32
    }

    fn accelerate(&mut self, search: &Search) {
        while let (Some((dest_a, prog_a)), Some((dest_b, prog_b))) = (self.a.dest, self.b.dest) {
            if self.step >= 26 {
                return;
            }
            self.volume += self.total_flow(search);
            self.step += 1;
            if prog_a > 1 {
                self.a.dest = Some((dest_a, prog_a - 1));
            } else {
                self.enabled |= 1 << dest_a;
                self.a.pos = dest_a;
                self.a.dest = None;
            }
            if prog_b > 1 {
                self.b.dest = Some((dest_b, prog_b - 1));
            } else {
                self.enabled |= 1 << dest_b;
                self.b.pos = dest_b;
                self.b.dest = None;
            }
        }
    }

    fn choices(&self, search: &Search) -> Vec<SearchState> {
        enum Choice {
            Continue,
            Idle,
            Open(u8, u8),
        }

        if self.step >= 26 {
            return vec![];
        }

        let build_choices = |e: &Entity, other: &Entity| {
            if e.dest.is_some() {
                return vec![Choice::Continue];
            } else {
                let mut out = vec![];
                for v in &search.system.flow_priority {
                    if (self.enabled >> v) & 1 == 0 {
                        let delay = search.system.valves[e.pos as usize].times[*v] as u8;

                        if let Some((p, _)) = other.dest {
                            if p as usize == *v {
                                continue;
                            }
                        }
                        if self.step + delay + 2 > 26 {
                        } else {
                            out.push(Choice::Open(*v as u8, delay));
                        }
                    }
                }
                if out.len() == 0 {
                    out.push(Choice::Idle);
                }
                return out;
            }
        };

        let mut out = vec![];
        let flow = self.total_flow(search);
        for my_choice in build_choices(&self.a, &self.b) {
            for ele_choice in build_choices(&self.b, &self.a) {
                if self.a.pos == self.b.pos {
                    if let Choice::Open(ta, _) = my_choice {
                        if let Choice::Open(tb, _) = ele_choice {
                            if tb <= ta {
                                continue;
                            }
                        }
                    }
                }

                let mut base = self.clone();

                base.volume += flow;
                base.step += 1;

                match my_choice {
                    Choice::Open(dest, len) => {
                        base.a.dest = Some((dest, len));
                    }
                    Choice::Idle => {}
                    Choice::Continue => {
                        let (dest, prog) = base.a.dest.unwrap();

                        if prog > 1 {
                            base.a.dest = Some((dest, prog - 1));
                        } else {
                            base.enabled |= 1 << dest;
                            base.a.pos = dest;
                            base.a.dest = None;
                        }
                    }
                }

                match ele_choice {
                    Choice::Open(dest, len) => {
                        base.b.dest = Some((dest, len));
                    }
                    Choice::Idle => {}
                    Choice::Continue => {
                        let (dest, prog) = base.b.dest.unwrap();

                        if prog > 1 {
                            base.b.dest = Some((dest, prog - 1));
                        } else {
                            base.enabled |= 1 << dest;
                            base.b.pos = dest;
                            base.b.dest = None;
                        }
                    }
                }

                base.calculate_reward(search);

                out.push(base);
            }
        }

        out
    }
}

struct Search {
    system: System,
    initial: SearchState,
}

impl SearchState {
    fn best_possible_outcome(&self, search: &Search) -> usize {
        let steps = (26 - self.step) as usize;

        let mut clone = self.clone();
        let mut out = self.volume as usize;

        let a_first = self.a.dest.map(|i| i.1 - 1).unwrap_or(0) as usize;
        let b_first = self.b.dest.map(|i| i.1 - 1).unwrap_or(0) as usize;

        let a_specific: Option<u8> = self.a.dest.map(|i| i.0);
        let b_specific: Option<u8> = self.b.dest.map(|i| i.0);

        let a_next = if let Some(ap) = a_specific {
            let mut lowest = None;
            for p in &search.system.flow_priority {
                if (clone.enabled >> p) & 1 == 0 && (*p != ap as usize) {
                    let time = search.system.valves[ap as usize].times[*p];
                    if let Some(v) = lowest {
                        if time < v {
                            lowest = Some(time);
                        }
                    } else {
                        lowest = Some(time);
                    }
                }
            }
            a_first + lowest.unwrap_or(1) + 1
        } else {
            a_first + 3
        };

        let b_next = if let Some(ap) = b_specific {
            let mut lowest = None;
            for p in &search.system.flow_priority {
                if (clone.enabled >> p) & 1 == 0 && (*p != ap as usize) {
                    let time = search.system.valves[ap as usize].times[*p];
                    if let Some(v) = lowest {
                        if time < v {
                            lowest = Some(time);
                        }
                    } else {
                        lowest = Some(time);
                    }
                }
            }
            b_first + lowest.unwrap_or(1) + 1
        } else {
            b_first + 3
        };

        let generic_assign = |clone: &mut SearchState| {
            for p in &search.system.flow_priority {
                if (clone.enabled >> p) & 1 == 0 {
                    clone.enabled |= 1 << p;
                    break;
                }
            }
        };

        for c in 0..steps {
            out += clone.total_flow(search) as usize;

            let a_trigger = c == a_first || ((c >= a_next) && ((c - a_next) % 3 == 0));
            let b_trigger = c == b_first || ((c >= b_next) && ((c - b_next) % 3 == 0));

            if a_trigger {
                if let Some(v) = a_specific {
                    if c == a_first {
                        clone.enabled |= 1 << v;
                    } else {
                        generic_assign(&mut clone);
                    }
                } else {
                    generic_assign(&mut clone);
                }
            }

            if b_trigger {
                if let Some(v) = b_specific {
                    if c == b_first {
                        clone.enabled |= 1 << v;
                    } else {
                        generic_assign(&mut clone);
                    }
                } else {
                    generic_assign(&mut clone);
                }
            }
        }

        out
    }
}

impl Search {
    fn search(&mut self) -> u32 {
        let mut heap = BinaryHeap::new();
        heap.push(self.initial);

        let mut best: Option<SearchState> = None;
        let mut c = 0_usize;
        let mut skipped = 0_usize;

        while let Some(mut ss) = heap.pop() {
            c += 1;
            if let Some(b) = best {
                let best_outcome = ss.best_possible_outcome(self);
                if b.volume >= best_outcome as u32 {
                    skipped += 1;
                    continue;
                }
            }

            ss.accelerate(&self);

            if ss.step == 26 {
                if let Some(s) = best {
                    if s.volume < ss.volume {
                        best = Some(ss);
                    }
                } else {
                    best = Some(ss);
                }
            } else {
                for c in ss.choices(self) {
                    heap.push(c);
                }
            }
        }

        println!("{}", c);
        // println!("{:?}", best);
        // println!("{:#018b}", best.unwrap().enabled);

        best.unwrap().volume
    }
}

pub fn day_16() -> (String, String) {
    let start = time::Instant::now();
    let f = read_to_string("input/day16.txt").unwrap();

    let re =
        Regex::new(r"^Valve (..) has flow rate=(\d+); tunnels* leads* to valves* (.+)$").unwrap();

    let mut mapping = HashMap::new();

    let mut get_mapping = |m: &str| {
        if let Some(i) = mapping.get(m) {
            *i
        } else {
            let i = mapping.len();
            mapping.insert(m.to_string(), i);
            i
        }
    };

    let mut valves = vec![];
    for l in f.lines().filter(|l| l.len() > 0) {
        let c = re.captures(l).unwrap();

        let valve = get_mapping(&c[1]);
        let flow = str::parse::<usize>(&c[2]).unwrap();
        let to = (&c[3])
            .split(", ")
            .map(&mut get_mapping)
            .collect::<Vec<usize>>();

        valves.push(Valve {
            num: valve,
            times: vec![],
            flow,
            connections: to,
        });
    }

    valves.sort_by(|va, vb| va.num.cmp(&vb.num));

    let mut s = System {
        valves,
        flow_priority: vec![],
    };
    s.precalc();

    let mut search = Search {
        system: s,
        initial: SearchState {
            a: Entity {
                pos: get_mapping("AA") as u8,
                dest: None,
            },
            b: Entity {
                pos: get_mapping("AA") as u8,
                dest: None,
            },
            reward: 0,
            enabled: 0,
            volume: 0,
            step: 0,
        },
    };

    (format!("{}", search.search()), "".to_string())
}
