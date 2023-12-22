use std::collections::{hash_map::Entry, HashMap, VecDeque};

const DATA: &str = include_str!("day20.txt");

pub fn part1() -> Option<()> {
    let mut intern = HashMap::new();
    let button = get_or_intern("button", &mut intern);
    let broadcaster = get_or_intern("broadcaster", &mut intern);
    let mut wires = parse(DATA, &mut intern);

    let mut pulses = (0, 0);
    for _ in 0..1000 {
        let (lo, hi) = pulse(&mut wires, button, broadcaster);
        pulses.0 += lo;
        pulses.1 += hi;
    }

    let result = pulses.0 * pulses.1;
    println!("result = {result}");
    Some(())
}

pub fn part2() -> Option<()> {
    let mut intern = HashMap::new();
    let button = get_or_intern("button", &mut intern);
    let broadcaster = get_or_intern("broadcaster", &mut intern);
    let goal = get_or_intern("rx", &mut intern);
    let wires = parse(DATA, &mut intern);

    let (goal_source_idx, goal_source) = wires.iter().find(|(_, w)| w.has_output(goal))?;

    let goal_source_inputs = goal_source.get_inputs()?;

    let presses = find_button_presses_for_hi(
        &wires,
        button,
        broadcaster,
        *goal_source_idx,
        goal_source_inputs,
    );

    let result = presses.into_iter().reduce(|acc, x| lcm(acc, x))?;
    println!("result = {result}");
    Some(())
}

type Interner<'a> = HashMap<&'a str, usize>;

fn get_or_intern<'a>(s: &'a str, intern: &mut Interner<'a>) -> usize {
    let next_idx = intern.len();
    match intern.entry(s) {
        Entry::Occupied(v) => *v.get(),
        Entry::Vacant(q) => {
            q.insert(next_idx);
            next_idx
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Wire {
    Broadcast {
        outputs: Vec<usize>,
    },
    Flip {
        on: bool,
        outputs: Vec<usize>,
    },
    Conj {
        memory: HashMap<usize, bool>,
        outputs: Vec<usize>,
    },
}

impl Wire {
    fn has_output(&self, target: usize) -> bool {
        match self {
            Wire::Broadcast { outputs } => outputs.contains(&target),
            Wire::Flip { outputs, .. } => outputs.contains(&target),
            Wire::Conj { outputs, .. } => outputs.contains(&target),
        }
    }

    fn get_inputs(&self) -> Option<Vec<usize>> {
        match self {
            Wire::Flip { .. } => None,
            Wire::Broadcast { .. } => None,
            Wire::Conj { memory, .. } => Some(memory.keys().cloned().collect()),
        }
    }
}

fn parse<'a>(input: &'a str, intern: &mut Interner<'a>) -> HashMap<usize, Wire> {
    let mut result = HashMap::new();
    let mut conjs = Vec::new();
    for line in input.lines() {
        let (left, right) = line.split_once(" -> ").unwrap();
        let outputs = right
            .split(", ")
            .map(|s| get_or_intern(s, intern))
            .collect::<Vec<_>>();
        if left == "broadcaster" {
            let name = get_or_intern("broadcaster", intern);
            result.insert(name, Wire::Broadcast { outputs });
        } else {
            let sym = left.chars().next().unwrap();
            let name = left.trim_start_matches("%").trim_start_matches("&");
            let name = get_or_intern(name, intern);
            if sym == '%' {
                result.insert(name, Wire::Flip { on: false, outputs });
            } else {
                result.insert(
                    name,
                    Wire::Conj {
                        memory: HashMap::new(),
                        outputs,
                    },
                );
                conjs.push(name);
            };
        }
    }
    for conj in conjs {
        let mut inputs = Vec::new();
        for (n, w) in result.iter() {
            match w {
                Wire::Conj { outputs, .. }
                | Wire::Flip { outputs, .. }
                | Wire::Broadcast { outputs } => {
                    if outputs.contains(&conj) {
                        inputs.push(*n);
                    }
                }
            }
        }
        for input in inputs {
            if let Some(Wire::Conj { memory, .. }) = result.get_mut(&conj) {
                memory.insert(input, false);
            }
        }
    }

    result
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pulse {
    from: usize,
    to: usize,
    lo: bool,
}

fn find_button_presses_for_hi(
    wires: &HashMap<usize, Wire>,
    button: usize,
    broadcaster: usize,
    goal_source_idx: usize,
    goals: Vec<usize>,
) -> Vec<usize> {
    let mut these_wires = wires.clone();
    let mut presses = 0;
    let mut results = HashMap::new();
    for goal in goals {
        results.insert(goal, None);
    }
    loop {
        presses += 1;
        cycle_pulse(
            &mut these_wires,
            button,
            broadcaster,
            presses,
            goal_source_idx,
            &mut results,
        );
        if results.values().all(|v| v.is_some()) {
            break;
        }
    }
    results.values().map(|v| v.unwrap()).collect()
}

fn cycle_pulse(
    wires: &mut HashMap<usize, Wire>,
    button: usize,
    broadcaster: usize,
    press: usize,
    target: usize,
    seen: &mut HashMap<usize, Option<usize>>,
) {
    let mut pulses = VecDeque::new();
    pulses.push_back(Pulse {
        from: button,
        to: broadcaster,
        lo: true,
    });
    while !pulses.is_empty() {
        let curr = pulses.pop_front().unwrap();
        if curr.to == target && !curr.lo {
            seen.insert(curr.from, Some(press));
        }

        match wires.get_mut(&curr.to) {
            None => {}
            Some(Wire::Broadcast { outputs }) => {
                for o in outputs {
                    pulses.push_back(Pulse {
                        from: broadcaster,
                        to: *o,
                        lo: curr.lo,
                    });
                }
            }
            Some(Wire::Flip { on, outputs }) => {
                if curr.lo {
                    *on = !*on;
                    for o in outputs {
                        pulses.push_back(Pulse {
                            from: curr.to,
                            to: *o,
                            lo: !*on,
                        });
                    }
                }
            }
            Some(Wire::Conj { memory, outputs }) => {
                let e = memory.entry(curr.from).or_insert(false);
                *e = !curr.lo;
                let send_lo = memory.values().all(|v| *v);
                for o in outputs {
                    pulses.push_back(Pulse {
                        from: curr.to,
                        to: *o,
                        lo: send_lo,
                    });
                }
            }
        }
    }
}

fn pulse(wires: &mut HashMap<usize, Wire>, button: usize, broadcaster: usize) -> (u32, u32) {
    let mut lo = 0;
    let mut hi = 0;

    let mut pulses = VecDeque::new();
    pulses.push_back(Pulse {
        from: button,
        to: broadcaster,
        lo: true,
    });
    while !pulses.is_empty() {
        let curr = pulses.pop_front().unwrap();
        if curr.lo {
            lo += 1;
        } else {
            hi += 1;
        }

        match wires.get_mut(&curr.to) {
            None => {}
            Some(Wire::Broadcast { outputs }) => {
                for o in outputs {
                    pulses.push_back(Pulse {
                        from: broadcaster,
                        to: *o,
                        lo: curr.lo,
                    });
                }
            }
            Some(Wire::Flip { on, outputs }) => {
                if curr.lo {
                    *on = !*on;
                    for o in outputs {
                        pulses.push_back(Pulse {
                            from: curr.to,
                            to: *o,
                            lo: !*on,
                        });
                    }
                }
            }
            Some(Wire::Conj { memory, outputs }) => {
                let e = memory.entry(curr.from).or_insert(false);
                *e = !curr.lo;
                let send_lo = memory.values().all(|v| *v);
                for o in outputs {
                    pulses.push_back(Pulse {
                        from: curr.to,
                        to: *o,
                        lo: send_lo,
                    });
                }
            }
        }
    }

    (lo, hi)
}

fn lcm(a: usize, b: usize) -> usize {
    let g = gcd(a, b);
    let bb = b / g;
    a * bb
}

fn gcd(a: usize, b: usize) -> usize {
    let (mut x, mut y) = if a > b { (a, b) } else { (b, a) };
    while x != y {
        let t = x - y;
        if t > y {
            x = t;
        } else {
            x = y;
            y = t;
        }
    }
    x
}
