const DATA: &str = include_str!("day9.txt");

pub fn part1() -> Option<()> {
    let mut result = 0;
    for line in DATA.lines() {
        let nums = line
            .split_whitespace()
            .map(|v| v.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let n = compute_next(nums);
        result += n;
    }
    println!("result = {result}");
    Some(())
}

pub fn part2() {
    let mut result = 0;
    for line in DATA.lines() {
        let nums = line
            .split_whitespace()
            .map(|v| v.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let n = compute_prev(nums);
        result += n;
    }
    println!("result = {result}");
}

fn compute_prev(nums: Vec<isize>) -> isize {
    let mut firsts = Vec::new();
    let mut curr = nums;
    let mut n = Vec::new();
    let mut done = false;
    while !done {
        firsts.push(curr[0]);
        done = true;
        for v in curr.windows(2) {
            let d = v[1] - v[0];
            if d != 0 {
                done = false;
            }
            n.push(d);
        }
        curr = n;
        n = Vec::new();
    }
    let mut v = 0;
    for k in 0..firsts.len() {
        v = firsts[firsts.len() - k - 1] - v;
    }
    v
}

fn compute_next(nums: Vec<isize>) -> isize {
    let mut firsts = Vec::new();
    let mut curr = nums;
    let mut n = Vec::new();
    let mut done = false;
    while !done {
        firsts.push(curr[0]);
        done = true;
        for v in curr.windows(2) {
            let d = v[1] - v[0];
            if d != 0 {
                done = false;
            }
            n.push(d);
        }
        curr = n;
        n = Vec::new();
    }
    curr.push(0);
    n.clear();
    for k in 0..firsts.len() {
        n.push(firsts[firsts.len() - k - 1]);
        for j in 0..curr.len() {
            n.push(n[j] + curr[j]);
        }
        curr = n;
        n = Vec::new();
    }
    curr[curr.len() - 1]
}
