const DATA: &str = include_str!("day15.txt");

#[derive(Debug, Clone)]
enum Instr<'a> {
    Add {
        orig: &'a str,
        label: &'a str,
        h: u32,
        focal: u32,
    },
    Remove {
        orig: &'a str,
        label: &'a str,
        h: u32,
    },
}

impl<'a> Instr<'a> {
    fn hash(&self) -> u32 {
        match self {
            Instr::Add { orig, .. } => hash(orig),
            Instr::Remove { orig, .. } => hash(orig),
        }
    }
}

type Lens<'a> = (&'a str, u32);

struct HMap<'a> {
    buckets: Vec<Vec<Lens<'a>>>,
}

impl<'a> HMap<'a> {
    fn new() -> Self {
        let mut s = HMap {
            buckets: Vec::with_capacity(256),
        };
        for _ in 0..256 {
            s.buckets.push(Vec::new());
        }
        s
    }

    fn perform(&mut self, instr: Instr<'a>) {
        match instr {
            Instr::Add {
                label, h, focal, ..
            } => {
                let bucket = self.buckets.get_mut(h as usize).expect("missing bucket");
                match bucket.iter_mut().find(|l| l.0 == label) {
                    Some(lens) => lens.1 = focal,
                    None => bucket.push((label, focal)),
                }
            }
            Instr::Remove { label, h, .. } => {
                let bucket = self.buckets.get_mut(h as usize).expect("missing bucket");
                bucket.retain(|l| l.0 != label);
            }
        }
    }

    fn power(&self) -> u32 {
        let mut result = 0;
        for (i, b) in self.buckets.iter().enumerate() {
            for (j, lens) in b.iter().enumerate() {
                result += (i as u32 + 1) * (j as u32 + 1) * lens.1;
            }
        }
        result
    }
}

pub fn part1() -> Option<()> {
    let instrs = parse(DATA);
    let mut result = 0;
    for instr in instrs.into_iter() {
        result += instr.hash();
    }
    println!("result = {result}");
    Some(())
}

pub fn part2() -> Option<()> {
    let instrs = parse(DATA);
    let mut hm = HMap::new();
    for instr in instrs.into_iter() {
        hm.perform(instr);
    }
    let result = hm.power();
    println!("result = {result}");
    Some(())
}

fn parse(input: &str) -> Vec<Instr<'_>> {
    input
        .trim()
        .split(",")
        .map(|s| {
            if s.ends_with("-") {
                let label = s.trim_end_matches("-");
                Instr::Remove {
                    orig: s,
                    label,
                    h: hash(label),
                }
            } else {
                let (label, length) = s.split_once("=").unwrap();
                Instr::Add {
                    orig: s,
                    label,
                    h: hash(label),
                    focal: length.parse::<u32>().unwrap(),
                }
            }
        })
        .collect()
}

fn hash(s: &str) -> u32 {
    let mut h = 0;
    for c in s.chars() {
        h += c as u32;
        h *= 17;
        h %= 256;
    }
    h
}
