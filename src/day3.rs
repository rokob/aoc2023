const DATA: &str = include_str!("day3.txt");
use std::collections::{HashMap, HashSet};

enum Elem {
    Num(usize, u32),
    Symbol(char),
}

enum State {
    None,
    Num(usize, u32),
}

pub fn part1() -> Option<()> {
    let mut grid = HashMap::new();
    let mut symbols = HashSet::new();
    let mut row = 0;
    let mut state = State::None;
    let mut cols = 0;
    let mut id = 0;
    for line in DATA.lines() {
        state = State::None;
        for (col, c) in line.chars().enumerate() {
            cols = std::cmp::max(col, cols);
            if c.is_digit(10) {
                state = match state {
                    State::None => State::Num(col, c.to_digit(10)?),
                    State::Num(start, acc) => State::Num(start, acc * 10 + c.to_digit(10)?),
                };
            } else {
                if let State::Num(start, v) = state {
                    state = State::None;
                    for cc in start..col {
                        grid.insert((row, cc), Elem::Num(id, v));
                    }
                    id += 1;
                }
                if c != '.' {
                    grid.insert((row, col), Elem::Symbol(c));
                    symbols.insert((row, col));
                }
            }
        }
        if let State::Num(start, v) = state {
            state = State::None;
            for cc in start..=cols {
                grid.insert((row, cc), Elem::Num(id, v));
            }
            id += 1;
        }
        row += 1;
    }
    if let State::Num(start, v) = state {
        for cc in start..=cols {
            grid.insert((row, cc), Elem::Num(id, v));
        }
    }

    let mut parts = Vec::new();
    for (r, c) in symbols.into_iter() {
        let mut seen = HashSet::new();
        for r_offset in -1..2 {
            for c_offset in -1..2 {
                let rr = r as i32 + r_offset;
                let cc = c as i32 + c_offset;
                if rr >= 0 && cc >= 0 {
                    if let Some(Elem::Num(id, v)) = grid.get(&(rr as usize, cc as usize)) {
                        if seen.insert(*id) {
                            parts.push(*v)
                        }
                    }
                }
            }
        }
    }

    let result: u32 = parts.iter().sum();
    println!("result = {result}");

    Some(())
}

pub fn part2() -> Option<()> {
    let mut grid = HashMap::new();
    let mut symbols = HashSet::new();
    let mut row = 0;
    let mut state = State::None;
    let mut cols = 0;
    let mut id = 0;
    for line in DATA.lines() {
        state = State::None;
        for (col, c) in line.chars().enumerate() {
            cols = std::cmp::max(col, cols);
            if c.is_digit(10) {
                state = match state {
                    State::None => State::Num(col, c.to_digit(10)?),
                    State::Num(start, acc) => State::Num(start, acc * 10 + c.to_digit(10)?),
                };
            } else {
                if let State::Num(start, v) = state {
                    state = State::None;
                    for cc in start..col {
                        grid.insert((row, cc), Elem::Num(id, v));
                    }
                    id += 1;
                }
                if c != '.' {
                    grid.insert((row, col), Elem::Symbol(c));
                    symbols.insert((row, col, c));
                }
            }
        }
        if let State::Num(start, v) = state {
            state = State::None;
            for cc in start..=cols {
                grid.insert((row, cc), Elem::Num(id, v));
            }
            id += 1;
        }
        row += 1;
    }
    if let State::Num(start, v) = state {
        for cc in start..=cols {
            grid.insert((row, cc), Elem::Num(id, v));
        }
    }

    let mut ratios = Vec::new();
    for (r, c, sym) in symbols.into_iter() {
        if sym != '*' {
            continue;
        }
        let mut nums = Vec::new();
        let mut seen = HashSet::new();
        for r_offset in -1..2 {
            for c_offset in -1..2 {
                let rr = r as i32 + r_offset;
                let cc = c as i32 + c_offset;
                if rr >= 0 && cc >= 0 {
                    if let Some(Elem::Num(id, v)) = grid.get(&(rr as usize, cc as usize)) {
                        if seen.insert(*id) {
                            nums.push(*v)
                        }
                    }
                }
            }
        }
        if nums.len() == 2 {
            ratios.push(nums[0] * nums[1]);
        }
    }

    let result: u32 = ratios.iter().sum();
    println!("result = {result}");

    Some(())
}
