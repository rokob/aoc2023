use std::collections::HashSet;

const DATA: &str = include_str!("day11.txt");

pub fn part1() -> Option<()> {
    let mut result = 0;
    let mut space = Vec::new();
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();
    let mut galaxies = HashSet::new();
    for line in DATA.lines() {
        let mut row = Vec::new();
        let mut col = 0;
        for c in line.chars() {
            let galaxy = c == '#';
            row.push(galaxy);
            if galaxy {
                galaxies.insert((space.len(), col));
            }
            col += 1;
        }
        space.push(row);
    }
    for (i, r) in space.iter().enumerate() {
        if !r.iter().any(|x| *x) {
            empty_rows.push(i);
        }
    }
    'outer: for c in 0..space[0].len() {
        for r in 0..space.len() {
            if space[r][c] {
                continue 'outer;
            }
        }
        empty_cols.push(c);
    }
    let mut reals = Vec::new();
    for (r, c) in galaxies.iter() {
        let rr = r + empty_rows.iter().take_while(|i| **i < *r).count();
        let cc = c + empty_cols.iter().take_while(|i| **i < *c).count();
        reals.push((rr, cc));
    }
    for i in 0..reals.len() {
        for j in i + 1..reals.len() {
            let x = reals[i];
            let y = reals[j];
            let dist = x.0.abs_diff(y.0) + x.1.abs_diff(y.1);
            result += dist;
        }
    }
    println!("result = {result}");
    Some(())
}

pub fn part2() -> Option<()> {
    let mut result = 0;
    let mut space = Vec::new();
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();
    let mut galaxies = HashSet::new();
    for line in DATA.lines() {
        let mut row = Vec::new();
        let mut col = 0;
        for c in line.chars() {
            let galaxy = c == '#';
            row.push(galaxy);
            if galaxy {
                galaxies.insert((space.len(), col));
            }
            col += 1;
        }
        space.push(row);
    }
    for (i, r) in space.iter().enumerate() {
        if !r.iter().any(|x| *x) {
            empty_rows.push(i);
        }
    }
    'outer: for c in 0..space[0].len() {
        for r in 0..space.len() {
            if space[r][c] {
                continue 'outer;
            }
        }
        empty_cols.push(c);
    }
    let mut reals = Vec::new();
    for (r, c) in galaxies.iter() {
        let rr = r + empty_rows.iter().take_while(|i| **i < *r).count() * (1000000 - 1);
        let cc = c + empty_cols.iter().take_while(|i| **i < *c).count() * (1000000 - 1);
        reals.push((rr, cc));
    }
    for i in 0..reals.len() {
        for j in i + 1..reals.len() {
            let x = reals[i];
            let y = reals[j];
            let dist = x.0.abs_diff(y.0) + x.1.abs_diff(y.1);
            result += dist;
        }
    }
    println!("result = {result}");
    Some(())
}
