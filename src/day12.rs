const DATA: &str = include_str!("day12.txt");
use std::collections::HashMap;

pub fn part1() -> Option<()> {
    let mut result = 0;
    for line in DATA.lines() {
        let ans = arrangements(line, 1)?;
        result += ans;
    }
    println!("result = {result}");
    Some(())
}

pub fn part2() -> Option<()> {
    let mut result = 0;
    for line in DATA.lines() {
        result += arrangements(line, 5)?;
    }
    println!("result = {result}");
    Some(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Spring {
    Oper,
    Broken,
    Unknown,
}

impl Spring {
    fn is_broken(&self) -> bool {
        *self == Spring::Broken
    }

    fn is_maybe_broken(&self) -> bool {
        match self {
            Spring::Broken | Spring::Unknown => true,
            _ => false,
        }
    }

    fn is_maybe_operational(&self) -> bool {
        match self {
            Spring::Oper | Spring::Unknown => true,
            _ => false,
        }
    }
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '#' => Spring::Broken,
            '.' => Spring::Oper,
            '?' => Spring::Unknown,
            _ => panic!("bad input: {c}"),
        }
    }
}

fn parse_input(input: &str) -> Option<(Vec<Spring>, Vec<usize>)> {
    let (spring_data, count_data) = input.split_once(' ')?;
    let counts = count_data
        .split(',')
        .flat_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let springs = spring_data.chars().map(From::from).collect::<Vec<Spring>>();

    Some((springs, counts))
}

fn expand(springs: Vec<Spring>, counts: Vec<usize>, size: usize) -> (Vec<Spring>, Vec<usize>) {
    let mut output_springs = springs.clone();
    let mut output_counts = counts.clone();
    for _ in 0..(size - 1) {
        output_springs.push(Spring::Unknown);
        output_springs.append(&mut springs.clone());
        output_counts.append(&mut counts.clone());
    }
    (output_springs, output_counts)
}

fn arrangements(input: &str, size: usize) -> Option<usize> {
    let (springs, counts) = parse_input(input)?;
    let (springs, counts) = expand(springs, counts, size);
    let mut cache = HashMap::new();
    Some(dp(springs.as_slice(), counts.as_slice(), &mut cache))
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Key<'a> {
    springs: &'a [Spring],
    counts: &'a [usize],
}

fn is_broken(springs: &[Spring], count: usize) -> bool {
    if springs.len() < count {
        return false;
    }
    for s in springs.iter().take(count) {
        if !s.is_maybe_broken() {
            return false;
        }
    }
    if springs.len() == count {
        true
    } else {
        !springs[count].is_broken()
    }
}

fn is_operational(springs: &[Spring]) -> bool {
    springs.iter().all(|&s| s.is_maybe_operational())
}

fn dp<'a>(
    springs: &'a [Spring],
    counts: &'a [usize],
    cache: &mut HashMap<Key<'a>, usize>,
) -> usize {
    if springs.is_empty() {
        return if counts.is_empty() { 1 } else { 0 };
    }
    if counts.is_empty() {
        return if is_operational(springs) { 1 } else { 0 };
    }

    let key = Key { springs, counts };
    if let Some(t) = cache.get(&key) {
        return *t;
    }

    let mut total = 0;
    if is_broken(springs, counts[0]) {
        let offset = if springs.len() == counts[0] {
            counts[0]
        } else {
            counts[0] + 1
        };
        total += dp(&springs[offset..], &counts[1..], cache);
    }
    if springs[0] == Spring::Oper || springs[0] == Spring::Unknown {
        total += dp(&springs[1..], counts, cache);
    }
    cache.insert(key, total);
    total
}
