use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
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
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ratio = |v: &SearchState| v.volume / (v.step.max(1) as u32);
        // Some(ratio(self).cmp(&ratio(other)))
        //Some(self.volume.cmp(&other.volume))
        //Some(other.step.cmp(&self.step))
        Some(self.step.cmp(&other.step))
        // Some(
        //     other
        //         .step
        //         .cmp(&self.step)
        //         .then(other.volume.cmp(&self.volume)),
        // )
    }
}

impl SearchState {
    // fn essential(&self, optimistic: bool) -> (u8, u8, u8, u64) {
    //     let mut res = (self.step, self.a.pos, self.b.pos, self.enabled);
    //     if optimistic {
    //         if let Some((dst, b)) = self.a.dest {
    //             res.1 = dst;
    //         }
    //         if let Some((dst, b)) = self.b.dest {
    //             res.2 = dst;
    //         }
    //     }
    //     res
    // }

    fn steel_man(&self) -> SearchState {
        let mut res = self.clone();
        if let Some((dst, b)) = res.a.dest {
            res.a.pos = dst;
            res.a.dest = None;
            res.enabled |= (1 << dst);
        }
        if let Some((dst, b)) = res.b.dest {
            res.b.pos = dst;
            res.b.dest = None;
            res.enabled |= (1 << dst);
        }
        res
    }

    fn strictly_superior_to(&self, search: &Search, other: &SearchState) -> bool {
        if other.step < self.step {
            return false;
        }

        let vol_adjust = self.volume + self.total_flow(search) * (other.step - self.step) as u32;

        vol_adjust >= other.volume
    }

    fn total_flow(&self, search: &Search) -> u32 {
        let mut out = 0;
        for x in 0..64 {
            if (self.enabled >> x) & 1 == 1 {
                let valve = &search.system.valves[x];
                out += valve.flow;
            }
        }
        out as u32
    }

    fn choices(&self, search: &Search) -> Vec<SearchState> {
        enum Choice {
            Continue,
            Idle,
            Open(u8, u8),
        };

        if self.step >= 26 {
            return vec![];
        }

        let build_choices = |e: &Entity| {
            if e.dest.is_some() {
                return vec![Choice::Continue];
            } else {
                let mut out = vec![];
                for v in &search.system.flow_priority {
                    if (self.enabled >> v) & 1 == 0 {
                        let delay = search.system.valves[e.pos as usize].times[*v] as u8;

                        if self.step + delay + 2 > 26 {
                        } else {
                            out.push(Choice::Open(
                                *v as u8, delay,
                                //+ 1,
                            ));
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
        for my_choice in build_choices(&self.a) {
            for ele_choice in build_choices(&self.b) {
                let mut base = self.clone();
                base.volume += flow;
                base.step += 1;
                //
                // let mut apply = |entity:&Entity,choice:&Choice| {
                //
                // };

                //apply(&mut self.a, &mut self.b);

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
                            base.enabled |= (1 << dest);
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
                            base.enabled |= (1 << dest);
                            base.b.pos = dest;
                            base.b.dest = None;
                        }
                    }
                }

                //
                // if let Choice::Move(sq) = my_choice {
                //     base.a.pos = sq;
                // } else {
                //     base.enabled |= (1 << self.a.pos);
                // }
                //
                // if let Choice::Move(sq) = ele_choice {
                //     base.b.pos = sq;
                // } else {
                //     base.enabled |= (1 << self.b.pos);
                // }

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
    //

    fn best_possible_outcome(&self, search: &Search) -> usize {
        let steps = (26 - self.step) as usize;

        let mut clone = self.clone();
        let mut out = self.volume as usize;

        for c in 0..steps {
            out += clone.total_flow(search) as usize;

            if c % 3 == 0 {
                let mut applied = 0;
                for p in &search.system.flow_priority {
                    if (clone.enabled >> p) & 1 == 0 {
                        applied += 1;
                        if applied > 2 {
                            break;
                        }
                        clone.enabled |= (1 << p);
                    }
                }
            }
        }

        //println!("{:?}", (time::Instant::now() - start).as_micros());
        out

        // let steps = (26 - self.step) as usize;
        // let mut out = 0;
        // for v in &search.system.valves {
        //     out += v.flow * steps;
        // }
        // out + self.volume as usize
    }
}

impl Search {
    fn search(&mut self) -> u32 {
        let mut heap = BinaryHeap::new();
        heap.push(self.initial);

        let mut highest_scores = HashMap::<(u8, u8, u8, u64), usize>::new();

        let mut best_states = HashMap::<u64, SearchState>::new();

        let mut best: Option<SearchState> = None;
        let mut c = 0_usize;
        let mut skipped = 0_usize;
        let mut skipped_best = 0_usize;

        while let Some(ss) = heap.pop() {
            //println!("{} {}", ss.step, ss.volume);

            c += 1;
            if c % 1_000_000 == 0 {
                println!("{} {} {} {}", heap.len(), skipped, skipped_best, c);
                println!(
                    "Best {} {} {}",
                    ss.step,
                    ss.best_possible_outcome(self),
                    best.map(|v| v.volume).unwrap_or(0)
                );
                println!("BSL: {}", best_states.len());
            }

            // if let Some(s) = highest_scores.get(&ss.essential(true)) {
            //     if *s >= ss.volume as usize {
            //         skipped_best += 1;
            //         continue;
            //     }
            // }
            //
            // highest_scores.insert(ss.essential(false), ss.volume as usize);

            // let steel = ss.steel_man();
            // if let Some(s) = best_states.get(&steel.enabled) {
            //     if s.strictly_superior_to(&self, &ss) {
            //         skipped_best += 1;
            //         continue;
            //     }
            // }
            //
            // if let Some(s) = best_states.get(&ss.enabled) {
            //     if ss.strictly_superior_to(&self, s) {
            //         best_states.insert(ss.enabled, ss.clone());
            //     }
            // } else {
            //     best_states.insert(ss.enabled, ss.clone());
            // }

            if let Some(b) = best {
                let best_outcome = ss.best_possible_outcome(self);
                if b.volume >= best_outcome as u32 {
                    skipped += 1;
                    continue;
                }
            }

            if ss.step == 26 {
                if let Some(s) = best {
                    if s.volume < ss.volume {
                        //println!("new best {}", ss.volume);
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
            //println!("{}", ss.volume);
            //println!("{:?}", heap);
        }

        println!("{:?}", best);
        println!("{:#018b}", best.unwrap().enabled);

        best.unwrap().volume
        //todo!();
    }

    // fn search(&mut self) -> u32 {
    //     let mut next = vec![self.initial];
    //
    //     loop {
    //         let mut step = HashMap::<(u8, u8, u64), SearchState>::new();
    //         for n in &next {
    //             for c in n.choices(self) {
    //                 if let Some(oc) = step.get(&c.essential()) {
    //                     if c.volume > oc.volume {
    //                         step.insert(c.essential(), c);
    //                     }
    //                 } else {
    //                     step.insert(c.essential(), c);
    //                 }
    //             }
    //         }
    //
    //         println!("{}", step.len());
    //
    //         if step.len() == 0 {
    //             break;
    //         }
    //
    //         next = Vec::from_iter(step.values().map(|v| *v));
    //     }
    //
    //     next.sort_by(|va, vb| va.volume.cmp(&vb.volume));
    //
    //     next.last().unwrap().volume
    // }
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
            println!("{} {}", i, m);
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
            enabled: 0,
            volume: 0,
            step: 0,
        },
    };

    (format!("{}", search.search()), "".to_string());
    println!("{:?}", (time::Instant::now() - start).as_micros());
    panic!();
}
