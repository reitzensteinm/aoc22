use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Valve {
    num: usize,
    flow: usize,
    connections: Vec<usize>,
    times: Vec<usize>,
}

#[derive(Clone)]
struct System {
    valves: Vec<Valve>,
    flow_priority: Vec<usize>,
}

impl System {
    // Precached pathfind of the shortest path from any valve to any other valve
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

    // List of the valves that have flow, sorted from high flow to low
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

impl Entity {
    fn partial_reward(&self) -> usize {
        if let Some((score, dist)) = self.dest {
            score as usize / dist as usize
        } else {
            0
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord)]
struct SearchState {
    player: Entity,
    elephant: Entity,
    enabled: u64,
    volume: u32,
    flow: u32,
    step: u8,
    reward: usize,
}

// We search depth first, but sort candidates of equal day to find the most promising
// The most fruitful elimination of nodes in the tree appears to by culling intermediate states
// based on their best possible outcome compared to the known best solution.
// Intermediate states contain a lot of data - 2x positions, valves and current volume. This puts
// dynamic programming off the table, and makes comparing intermediate states with each other
// for culling difficult considering we only end up visiting about 400k of them.
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
    fn open_valve(&mut self, search: &Search, v: u8) {
        debug_assert_eq!((self.enabled >> v) & 1, 0);
        self.enabled |= 1 << v;
        self.flow += search.system.valves[v as usize].flow as u32;
    }

    // This is an indication of how promising a search state is
    fn calculate_reward(&mut self) {
        let mut reward = self.flow as usize;

        reward += self.player.partial_reward();
        reward += self.elephant.partial_reward();

        self.reward = reward;
    }

    // Builds possible next states from the current state, culling where it can
    fn next_states(&self, search: &Search) -> Vec<SearchState> {
        enum Choice {
            Continue,
            Idle,
            Open(u8, u8),
        }

        if self.step >= search.steps as u8 {
            return vec![];
        }

        // Plans out the next move for each entity
        let build_choices = |e: &Entity, other: &Entity| {
            // If pathfinding is underway, continue
            if e.dest.is_some() {
                return vec![Choice::Continue];
            } else {
                let mut out = vec![];
                for v in &search.system.flow_priority {
                    // Check valves that aren't enabled, that aren't the target of the other entity
                    if (self.enabled >> v) & 1 == 0 {
                        let delay = search.system.valves[e.pos as usize].times[*v] as u8;

                        if let Some((p, _)) = other.dest {
                            if p as usize == *v {
                                continue;
                            }
                        }

                        // If we won't get there and turn it on and leave it on for a turn,
                        // it can't help
                        if self.step + delay + 2 > search.steps as u8 {
                        } else {
                            out.push(Choice::Open(*v as u8, delay));
                        }
                    }
                }
                // When an entity has nothing to do, swap to idle mode
                if out.len() == 0 {
                    out.push(Choice::Idle);
                }
                return out;
            }
        };

        let my_choices = build_choices(&self.player, &self.elephant);
        // To support Part A
        let elephant_choices = if search.elephant {
            build_choices(&self.elephant, &self.player)
        } else {
            vec![Choice::Idle]
        };

        let mut out = vec![];

        for my_choice in &my_choices {
            for ele_choice in &elephant_choices {
                if let Choice::Open(ta, _) = my_choice {
                    if let Choice::Open(tb, _) = ele_choice {
                        if self.player.pos == self.elephant.pos {
                            // This prevents mirrored opening moves, since the two entities
                            // are identical.
                            if *tb <= *ta {
                                continue;
                            }
                        }

                        // Both entities shouldn't move to the same square.
                        if *ta == *tb {
                            continue;
                        }
                    }
                }

                let mut base = self.clone();

                base.volume += base.flow;
                base.step += 1;

                // Entities can either be moving towards and unlocking a goal, idling,
                // or locking on to a new goal
                let apply_choice = |choice: &Choice, entity: &mut Entity| match *choice {
                    Choice::Open(dest, len) => {
                        entity.dest = Some((dest, len));
                        None
                    }
                    Choice::Idle => None,
                    Choice::Continue => {
                        let (dest, prog) = entity.dest.unwrap();
                        if prog > 1 {
                            entity.dest = Some((dest, prog - 1));
                            None
                        } else {
                            entity.pos = dest;
                            entity.dest = None;
                            Some(dest)
                        }
                    }
                };

                // Duplicated due to aliasing rules
                if let Some(v) = apply_choice(my_choice, &mut base.player) {
                    base.open_valve(search, v);
                }

                if let Some(v) = apply_choice(ele_choice, &mut base.elephant) {
                    base.open_valve(search, v);
                }

                // Caches the expected value of this state
                base.calculate_reward();

                out.push(base);
            }
        }

        out
    }
}

struct Search {
    steps: usize,
    elephant: bool,

    checked: usize,
    skipped: usize,
    system: System,
    best: Option<SearchState>,
    initial: SearchState,
}

impl SearchState {
    // This is where most of the magic happens. It extrapolates forward from the current state,
    // working out what the best possible volume it could yield. This allows for aggressive culling.
    // It's long and convoluted, because it:
    // * Finishes the current move of each entity
    // * Finds the fastest possible move it could make from dest to another valve
    // * From there, conservatively unlocks the best valves every three turns
    // While complex, it cuts the visited states from tens of millions to just 400k for part 2,
    // compared to a more naive solution, more than paying for its weight.
    fn best_possible_outcome(&self, search: &Search) -> usize {
        let steps = search.steps - self.step as usize;

        let mut clone = self.clone();
        let mut out = self.volume as usize;

        let plan = |entity: &Entity| {
            let first = entity.dest.map(|i| i.1 - 1).unwrap_or(0) as usize;
            let specific: Option<u8> = entity.dest.map(|i| i.0);

            let next = if let Some(sp) = specific {
                let mut lowest = None;
                for p in &search.system.flow_priority {
                    if (clone.enabled >> p) & 1 == 0 && (*p != sp as usize) {
                        let time = search.system.valves[sp as usize].times[*p];
                        if let Some(v) = lowest {
                            if time < v {
                                lowest = Some(time);
                            }
                        } else {
                            lowest = Some(time);
                        }
                    }
                }
                first + lowest.unwrap_or(1) + 1
            } else {
                first + 3
            };

            // Yields an optimistic plan with:
            // * the turn on which the next valve will be unlocked
            // * the specific valve that turn unlocks (if any)
            // * the turn on which the valve after that will be unlocked
            (first, specific, next)
        };

        let (a_first, a_specific, a_next) = plan(&self.player);
        let (b_first, b_specific, b_next) = plan(&self.elephant);

        let generic_assign = |clone: &mut SearchState| {
            for p in &search.system.flow_priority {
                if *p == a_specific.unwrap_or(0) as usize || *p == b_specific.unwrap_or(0) as usize
                {
                    continue;
                }
                if (clone.enabled >> p) & 1 == 0 {
                    clone.open_valve(search, *p as u8);
                    break;
                }
            }
        };

        for c in 0..steps {
            out += clone.flow as usize;

            // Ugly, but refactoring these into a fn instead of duplicating slows the program
            // down by 25%. Suspect nested fns not inlined. I'm not going to break out Godbolt for AoC...
            let a_trigger = c == a_first || ((c >= a_next) && ((c - a_next) % 3 == 0));
            let b_trigger = c == b_first || ((c >= b_next) && ((c - b_next) % 3 == 0));

            if a_trigger {
                if let Some(v) = a_specific {
                    if c == a_first {
                        clone.open_valve(search, v);
                    } else {
                        generic_assign(&mut clone);
                    }
                } else {
                    generic_assign(&mut clone);
                }
            }

            if b_trigger && search.elephant {
                if let Some(v) = b_specific {
                    if c == b_first {
                        clone.open_valve(search, v);
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
    fn new(system: System, steps: usize, elephant: bool, start: u8) -> Search {
        Search {
            steps,
            elephant,
            checked: 0,
            skipped: 0,
            system,
            best: None,
            initial: SearchState {
                player: Entity {
                    pos: start,
                    dest: None,
                },
                elephant: Entity {
                    pos: start,
                    dest: None,
                },
                flow: 0,
                reward: 0,
                enabled: 0,
                volume: 0,
                step: 0,
            },
        }
    }

    // Searches recursively through states, sorting by expected value at each node
    // I found that, when using a priority queue, the best way to tune it was a depth
    // first search anyway. This replicates the logic with lower allocation since more
    // lives on the stack.
    fn search_inner(&mut self, state: SearchState) {
        let choices = state.next_states(self);
        self.checked += 1;

        for child in choices.into_iter().sorted().rev() {
            if let Some(best) = self.best {
                let best_outcome = child.best_possible_outcome(self);
                if best.volume >= best_outcome as u32 {
                    self.skipped += 1;
                    continue;
                }

                if best.volume < child.volume {
                    self.best = Some(child);
                }
            } else {
                self.best = Some(child);
            }

            self.search_inner(child);
        }
    }

    fn search(&mut self) -> u32 {
        let initial = self.initial;

        self.search_inner(initial);
        self.best.unwrap().volume
    }
}

pub fn day_16() -> (String, String) {
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

    let start = get_mapping("AA") as u8;

    let a = Search::new(s.clone(), 30, false, start).search();
    let b = Search::new(s, 26, true, start).search();

    (format!("{a}"), format!("{b}"))
}
