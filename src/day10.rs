use std::collections::{HashMap, VecDeque};

const DATA: &str = include_str!("day10.txt");
/*
 | is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Ns,
    Ew,
    Ne,
    Nw,
    Sw,
    Se,
    Start,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
            Dir::E => Dir::W,
        }
    }
}

impl Pipe {
    fn dirs(&self) -> Option<[Dir; 2]> {
        match self {
            Pipe::Ns => Some([Dir::N, Dir::S]),
            Pipe::Ne => Some([Dir::N, Dir::E]),
            Pipe::Ew => Some([Dir::E, Dir::W]),
            Pipe::Nw => Some([Dir::N, Dir::W]),
            Pipe::Sw => Some([Dir::W, Dir::S]),
            Pipe::Se => Some([Dir::E, Dir::S]),
            _ => None,
        }
    }

    fn can_connect(&self, other: &Pipe, dir: Dir) -> Option<bool> {
        let my_dirs = self.dirs()?;
        let other_dirs = other.dirs()?;
        if !my_dirs.contains(&dir) {
            return Some(false);
        }
        return Some(other_dirs.contains(&dir.opposite()));
    }
}

fn offset(pos: (usize, usize), rows: usize, cols: usize, dir: Dir) -> Option<(usize, usize)> {
    match dir {
        Dir::N => {
            if pos.0 == 0 {
                return None;
            }
            Some((pos.0 - 1, pos.1))
        }
        Dir::S => {
            if pos.0 == rows - 1 {
                return None;
            }
            Some((pos.0 + 1, pos.1))
        }
        Dir::E => {
            if pos.1 == cols - 1 {
                return None;
            }
            Some((pos.0, pos.1 + 1))
        }
        Dir::W => {
            if pos.1 == 0 {
                return None;
            }
            Some((pos.0, pos.1 - 1))
        }
    }
}

pub fn part1() -> Option<()> {
    let mut start_pos = (0_usize, 0_usize);
    let mut grid = Vec::new();
    let mut row_idx = 0_usize;
    for line in DATA.lines() {
        let mut col = 0_usize;
        let mut row = Vec::new();
        for c in line.chars() {
            match c {
                '|' => row.push(Pipe::Ns),
                '-' => row.push(Pipe::Ew),
                'L' => row.push(Pipe::Ne),
                'J' => row.push(Pipe::Nw),
                '7' => row.push(Pipe::Sw),
                'F' => row.push(Pipe::Se),
                'S' => row.push(Pipe::Start),
                '.' => row.push(Pipe::None),
                _ => panic!("bad input: {}", c),
            }
            if c == 'S' {
                start_pos = (row_idx, col);
            }
            col += 1;
        }
        grid.push(row);
        row_idx += 1;
    }
    for start_kind in [Pipe::Ns, Pipe::Ew, Pipe::Ne, Pipe::Nw, Pipe::Sw, Pipe::Se] {
        let result = find_loop(start_pos, start_kind, &grid);
        println!("result with {:?} = {result:?}", start_kind);
    }
    Some(())
}

pub fn part2() -> Option<()> {
    // From part 1 => Start = Sw
    let mut start_pos = (0_usize, 0_usize);
    let mut grid = Vec::new();
    let mut row_idx = 0_usize;
    for line in DATA.lines() {
        let mut col = 0_usize;
        let mut row = Vec::new();
        for c in line.chars() {
            match c {
                '|' => row.push(Pipe::Ns),
                '-' => row.push(Pipe::Ew),
                'L' => row.push(Pipe::Ne),
                'J' => row.push(Pipe::Nw),
                '7' => row.push(Pipe::Sw),
                'F' => row.push(Pipe::Se),
                'S' => row.push(Pipe::Start),
                '.' => row.push(Pipe::None),
                _ => panic!("bad input: {}", c),
            }
            if c == 'S' {
                start_pos = (row_idx, col);
            }
            col += 1;
        }
        grid.push(row);
        row_idx += 1;
    }
    let start_kind = Pipe::Sw;
    let pipe_loop = get_loop(start_pos, start_kind, &grid);
    let mut result = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    print_loop(rows, cols, &pipe_loop);
    let new_loop = find_answer(rows, cols, &pipe_loop, &grid, start_kind);
    println!("\n\n-----\n\n");
    print_answer(rows, cols, &new_loop);
    for r in 0..rows {
        for c in 0..cols {
            if let Some(&3) = new_loop.get(&(r, c)) {
                result += 1;
            }
        }
    }
    println!("result = {result}");

    Some(())
}

fn find_answer(
    rows: usize,
    cols: usize,
    pipe_loop: &HashMap<(usize, usize), i32>,
    grid: &Vec<Vec<Pipe>>,
    start_kind: Pipe,
) -> HashMap<(usize, usize), i32> {
    let mut helper = pipe_loop
        .iter()
        .map(|((r, c), _)| ((*r, *c), 1))
        .collect::<HashMap<(usize, usize), i32>>();

    for r in 0..rows {
        let mut norths = 0;
        for c in 0..cols {
            if let Some(&1) = helper.get(&(r, c)) {
                let p = grid[r][c];
                let p = if p == Pipe::Start { start_kind } else { p };
                let d = p.dirs().unwrap();
                if d.contains(&Dir::N) {
                    norths += 1;
                }
                continue;
            }
            if norths % 2 == 0 {
                helper.insert((r, c), 2);
            } else {
                helper.insert((r, c), 3);
            }
        }
    }
    helper
}

fn print_loop(rows: usize, cols: usize, pipe_loop: &HashMap<(usize, usize), i32>) {
    for r in 0..rows {
        for c in 0..cols {
            match pipe_loop.get(&(r, c)) {
                Some(0) => print!("S"),
                Some(_) => print!("X"),
                None => print!("."),
            }
        }
        println!("");
    }
}

fn print_answer(rows: usize, cols: usize, pipe_loop: &HashMap<(usize, usize), i32>) {
    for r in 0..rows {
        for c in 0..cols {
            match pipe_loop.get(&(r, c)) {
                Some(0) => print!(" "),
                Some(1) => print!("~"),
                Some(2) => print!("."),
                Some(3) => print!("*"),
                Some(_) => print!("_"),
                None => print!("."),
            }
        }
        println!("");
    }
}

fn find_loop(start: (usize, usize), start_kind: Pipe, grid: &Vec<Vec<Pipe>>) -> Option<i32> {
    let mut paths = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let rows = grid.len();
    let cols = grid[0].len();
    while !queue.is_empty() {
        let curr = queue.pop_front()?;
        let (p, dist) = if grid[curr.0][curr.1] == Pipe::Start {
            (start_kind, 0)
        } else {
            (grid[curr.0][curr.1], *paths.get(&(curr.0, curr.1)).unwrap())
        };
        for dir in [Dir::N, Dir::S, Dir::E, Dir::W] {
            if let Some((r, c)) = offset(curr, rows, cols, dir) {
                if let Some(true) = p.can_connect(&grid[r][c], dir) {
                    if !paths.contains_key(&(r, c)) {
                        paths.insert((r, c), dist + 1);
                        queue.push_back((r, c));
                    }
                }
            }
        }
    }
    paths.into_values().max()
}

fn get_loop(
    start: (usize, usize),
    start_kind: Pipe,
    grid: &Vec<Vec<Pipe>>,
) -> HashMap<(usize, usize), i32> {
    let mut paths = HashMap::new();
    let mut queue = VecDeque::new();
    paths.insert(start, 0);
    queue.push_back(start);
    let rows = grid.len();
    let cols = grid[0].len();
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        let (p, dist) = if grid[curr.0][curr.1] == Pipe::Start {
            (start_kind, 0)
        } else {
            (grid[curr.0][curr.1], *paths.get(&(curr.0, curr.1)).unwrap())
        };
        for dir in [Dir::N, Dir::S, Dir::E, Dir::W] {
            if let Some((r, c)) = offset(curr, rows, cols, dir) {
                if let Some(true) = p.can_connect(&grid[r][c], dir) {
                    if !paths.contains_key(&(r, c)) {
                        paths.insert((r, c), dist + 1);
                        queue.push_back((r, c));
                    }
                }
            }
        }
    }
    paths
}
