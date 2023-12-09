use std::collections::{HashMap, HashSet};

const DATA: &str = include_str!("day8.txt");

pub fn part1() -> Option<()> {
    let mut data = DATA.lines();
    let dirs = data.next()?.chars().collect::<Vec<_>>();
    let mut intern = HashMap::new();
    let mut map = HashMap::new();
    data.next();
    for line in data {
        let (start, rest) = line.split_once(" = ")?;
        let (left, right) = rest.split_once(", ")?;
        let (l, r) = (left.trim_start_matches("("), right.trim_end_matches(")"));
        let start_idx = intern.len();
        let start_e = intern.entry(start).or_insert(start_idx);
        let ss = *start_e;
        let left_idx = intern.len();
        let left_e = intern.entry(l).or_insert(left_idx);
        let ll = *left_e;
        let right_idx = intern.len();
        let right_e = intern.entry(r).or_insert(right_idx);
        let rr = *right_e;
        map.insert(ss, (ll, rr));
    }

    let goal = *intern.get("ZZZ")?;

    let mut curr = *intern.get("AAA")?;

    let mut dir_idx = 0;
    let mut count = 0;
    while curr != goal {
        let dir = dirs[dir_idx];
        let e = map.get(&curr)?;
        if dir == 'L' {
            curr = e.0;
        } else {
            curr = e.1;
        }
        dir_idx = (dir_idx + 1) % dirs.len();
        count += 1;
    }
    println!("result = {count}");

    Some(())
}

fn find_count(
    start: usize,
    goals: &HashSet<usize>,
    map: &HashMap<usize, (usize, usize)>,
    dirs: &Vec<char>,
) -> Option<(usize, usize)> {
    let mut dir_idx = 0;
    let mut count = 0;
    let mut curr = start;
    while !goals.contains(&curr) {
        let dir = dirs[dir_idx];
        let e = map.get(&curr)?;
        if dir == 'L' {
            curr = e.0;
        } else {
            curr = e.1;
        }
        dir_idx = (dir_idx + 1) % dirs.len();
        count += 1;
    }
    Some((curr, count))
}

pub fn part2() -> Option<()> {
    let mut data = DATA.lines();
    let dirs = data.next()?.chars().collect::<Vec<_>>();
    let mut intern = HashMap::new();
    let mut map = HashMap::new();
    data.next();
    let mut starts = Vec::new();
    let mut ends = HashSet::new();
    for line in data {
        let (start, rest) = line.split_once(" = ")?;
        let (left, right) = rest.split_once(", ")?;
        let (l, r) = (left.trim_start_matches("("), right.trim_end_matches(")"));
        let start_idx = intern.len();
        let start_e = intern.entry(start).or_insert(start_idx);
        let ss = *start_e;
        let left_idx = intern.len();
        let left_e = intern.entry(l).or_insert(left_idx);
        let ll = *left_e;
        let right_idx = intern.len();
        let right_e = intern.entry(r).or_insert(right_idx);
        let rr = *right_e;
        if start.ends_with("A") {
            starts.push(ss);
        }
        if start.ends_with("Z") {
            ends.insert(ss);
        }
        if l.ends_with("A") {
            starts.push(ll);
        }
        if l.ends_with("Z") {
            ends.insert(ll);
        }
        if r.ends_with("A") {
            starts.push(rr);
        }
        if r.ends_with("Z") {
            ends.insert(rr);
        }
        map.insert(ss, (ll, rr));
    }

    let mut counts = Vec::new();
    for s in starts.iter() {
        let (_, c) = find_count(*s, &ends, &map, &dirs)?;
        counts.push(c);
    }
    let result = counts.into_iter().reduce(|acc, x| lcm(acc, x))?;
    println!("result = {result}");

    Some(())
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
