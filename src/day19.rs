use std::collections::{hash_map::Entry, HashMap};

const DATA: &str = include_str!("day19.txt");

pub fn part1() -> Option<()> {
    let mut intern = HashMap::new();
    let accept = 0;
    let reject = 1;
    intern.insert("A", accept);
    intern.insert("R", reject);
    let (workflows, parts) = parse(&mut intern, DATA);

    let mut result = 0;
    let start = *intern.get("in").unwrap();
    for part in parts.into_iter() {
        match part.execute(&workflows, start, accept, reject) {
            Outcome::Accept(v) => result += v,
            Outcome::Reject => {}
        }
    }

    println!("result = {result}");
    Some(())
}

pub fn part2() -> Option<()> {
    let mut intern = HashMap::new();
    let accept = 0;
    let reject = 1;
    intern.insert("A", accept);
    intern.insert("R", reject);
    let (workflows, _) = parse(&mut intern, DATA);

    let start = *intern.get("in").unwrap();
    let start_range = PartRange {
        lo: [1, 1, 1, 1],
        hi: [4000, 4000, 4000, 4000],
    };
    let mut targets = vec![RangeTarget {
        range: start_range,
        target: start,
    }];
    let mut results = Vec::new();
    while !targets.is_empty() {
        let mut next_targets = Vec::new();
        for target in targets.iter() {
            match target.range.step(&workflows, target.target, accept, reject) {
                RangeStep::Reject => {}
                RangeStep::Accept(r) => results.push(r),
                RangeStep::Continue(mut ts) => next_targets.append(&mut ts),
            }
        }
        targets = next_targets;
    }

    let result: usize = results.iter().map(|r| r.size()).sum();
    println!("result = {result}");

    Some(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Instr {
    Less {
        rating: usize,
        constant: usize,
        target: usize,
    },
    Greater {
        rating: usize,
        constant: usize,
        target: usize,
    },
    Goto(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Workflow {
    instrs: Vec<Instr>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct RangeTarget {
    range: PartRange,
    target: usize,
}

impl Workflow {
    fn step(&self, range: &PartRange) -> Vec<RangeTarget> {
        let mut result = Vec::new();
        let mut curr = range.clone();
        for instr in self.instrs.iter() {
            match instr {
                Instr::Less {
                    rating,
                    constant,
                    target,
                } => {
                    let (lo_r, hi_r) = (curr.lo[*rating], curr.hi[*rating]);
                    if hi_r < *constant {
                        result.push(RangeTarget {
                            range: curr,
                            target: *target,
                        });
                        return result;
                    } else if lo_r < *constant {
                        let (mut a, mut b) = (curr.clone(), curr.clone());
                        a.hi[*rating] = *constant - 1;
                        b.lo[*rating] = *constant;
                        result.push(RangeTarget {
                            range: a,
                            target: *target,
                        });
                        curr = b;
                    }
                }
                Instr::Greater {
                    rating,
                    constant,
                    target,
                } => {
                    let (lo_r, hi_r) = (curr.lo[*rating], curr.hi[*rating]);
                    if lo_r > *constant {
                        result.push(RangeTarget {
                            range: curr,
                            target: *target,
                        });
                        return result;
                    } else if hi_r > *constant {
                        let (mut a, mut b) = (curr.clone(), curr.clone());
                        a.lo[*rating] = *constant + 1;
                        b.hi[*rating] = *constant;
                        result.push(RangeTarget {
                            range: a,
                            target: *target,
                        });
                        curr = b;
                    }
                }
                Instr::Goto(t) => {
                    result.push(RangeTarget {
                        range: curr,
                        target: *t,
                    });
                    return result;
                }
            }
        }
        result
    }

    fn execute(&self, part: &Part) -> usize {
        for instr in self.instrs.iter() {
            match instr {
                Instr::Less {
                    rating,
                    constant,
                    target,
                } => {
                    if part.ratings[*rating] < *constant {
                        return *target;
                    }
                }
                Instr::Greater {
                    rating,
                    constant,
                    target,
                } => {
                    if part.ratings[*rating] > *constant {
                        return *target;
                    }
                }
                Instr::Goto(t) => return *t,
            }
        }
        unreachable!();
    }
}

type Workflows = HashMap<usize, Workflow>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Outcome {
    Accept(usize),
    Reject,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Part {
    ratings: [usize; 4],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct PartRange {
    lo: [usize; 4],
    hi: [usize; 4],
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum RangeStep {
    Accept(PartRange),
    Reject,
    Continue(Vec<RangeTarget>),
}

impl PartRange {
    fn step(&self, workflows: &Workflows, start: usize, accept: usize, reject: usize) -> RangeStep {
        match start {
            x if x == accept => RangeStep::Accept(*self),
            x if x == reject => RangeStep::Reject,
            _ => RangeStep::Continue(workflows.get(&start).unwrap().step(self)),
        }
    }

    fn size(&self) -> usize {
        let mut result = 1;
        for i in 0..4 {
            if self.hi[i] < self.lo[i] {
                return 0;
            }
            result *= self.hi[i] - self.lo[i] + 1;
        }
        result
    }
}

impl Part {
    fn score(&self) -> usize {
        self.ratings.iter().sum()
    }

    fn execute(
        &self,
        workflows: &Workflows,
        start: usize,
        accept: usize,
        reject: usize,
    ) -> Outcome {
        let mut curr = start;
        while curr != accept && curr != reject {
            curr = workflows.get(&curr).unwrap().execute(self);
        }
        if curr == accept {
            Outcome::Accept(self.score())
        } else {
            Outcome::Reject
        }
    }
}

fn rating_to_idx(rating: char) -> usize {
    match rating {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => panic!("bad rating: {rating}"),
    }
}

fn get_or_intern<'a>(s: &'a str, intern: &mut HashMap<&'a str, usize>) -> usize {
    let next_idx = intern.len();
    match intern.entry(s) {
        Entry::Occupied(v) => *v.get(),
        Entry::Vacant(q) => {
            q.insert(next_idx);
            next_idx
        }
    }
}

fn parse<'a>(intern: &mut HashMap<&'a str, usize>, input: &'a str) -> (Workflows, Vec<Part>) {
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();
    let mut state = true;
    for line in input.lines() {
        if line.is_empty() {
            state = !state;
            continue;
        }
        if state {
            // parse workflow
            // px{a<2006:qkq,m>2090:A,rfg}
            let (name, rest) = line.split_once("{").unwrap();
            let name_idx = get_or_intern(name, intern);
            let mut instrs = Vec::new();
            for piece in rest.trim_end_matches("}").split(",") {
                if piece.contains(":") {
                    let (cond, target) = piece.split_once(":").unwrap();
                    let target = get_or_intern(target, intern);
                    if cond.contains("<") {
                        let (rating, constant) = cond.split_once("<").unwrap();
                        let rating = rating_to_idx(rating.chars().next().unwrap());
                        let constant = constant.parse::<usize>().unwrap();
                        instrs.push(Instr::Less {
                            rating,
                            constant,
                            target,
                        });
                    } else {
                        let (rating, constant) = cond.split_once(">").unwrap();
                        let rating = rating_to_idx(rating.chars().next().unwrap());
                        let constant = constant.parse::<usize>().unwrap();
                        instrs.push(Instr::Greater {
                            rating,
                            constant,
                            target,
                        });
                    }
                } else {
                    let target = get_or_intern(piece, intern);
                    instrs.push(Instr::Goto(target));
                }
            }
            workflows.insert(name_idx, Workflow { instrs });
            continue;
        }
        // parse part
        //{x=787,m=2655,a=1222,s=2876}
        let mut part = Part {
            ratings: [0, 0, 0, 0],
        };
        for rating in line
            .trim_start_matches("{")
            .trim_end_matches("}")
            .split(",")
        {
            let (c, val) = rating.split_once("=").unwrap();
            let idx = rating_to_idx(c.chars().next().unwrap());
            let val = val.parse::<usize>().unwrap();
            part.ratings[idx] = val;
        }
        parts.push(part);
    }
    (workflows, parts)
}
