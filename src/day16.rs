use std::collections::HashSet;
use std::sync::mpsc;
use std::thread;

const DATA: &str = include_str!("day16.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    MirrorForward, // '\'
    MirrorBack,    // '/'
    SplitterH,     // '-'
    SplitterV,     // '|'
}

type Grid = Vec<Vec<Tile>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn(&self, t: &Tile) -> Option<Dir> {
        use Dir::*;
        use Tile::*;
        match (self, t) {
            (_, Empty) => Some(*self),

            (Up, MirrorForward) => Some(Left),
            (Left, MirrorForward) => Some(Up),
            (Down, MirrorForward) => Some(Right),
            (Right, MirrorForward) => Some(Down),

            (Up, MirrorBack) => Some(Right),
            (Left, MirrorBack) => Some(Down),
            (Down, MirrorBack) => Some(Left),
            (Right, MirrorBack) => Some(Up),

            (Up, SplitterV) | (Down, SplitterV) => Some(*self),
            (Left, SplitterH) | (Right, SplitterH) => Some(*self),

            _ => None,
        }
    }

    fn split(&self, t: &Tile) -> Option<(Dir, Dir)> {
        match (self, t) {
            (Dir::Left, Tile::SplitterV) | (Dir::Right, Tile::SplitterV) => {
                Some((Dir::Up, Dir::Down))
            }
            (Dir::Up, Tile::SplitterH) | (Dir::Down, Tile::SplitterH) => {
                Some((Dir::Left, Dir::Right))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Step {
    One(Beam),
    Two(Beam, Beam),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Beam {
    r: usize,
    c: usize,
    d: Dir,
}

impl Beam {
    fn step(&self, grid: &Grid, rows: usize, cols: usize) -> Option<Step> {
        let (rr, cc) = match self.d {
            Dir::Up => (self.r - 1, self.c),
            Dir::Down => (self.r + 1, self.c),
            Dir::Left => (self.r, self.c - 1),
            Dir::Right => (self.r, self.c + 1),
        };

        if rr < 1 || cc < 1 || rr > rows || cc > cols {
            return None;
        }

        let next_tile = grid[rr - 1][cc - 1];
        if let Some(new_dir) = self.d.turn(&next_tile) {
            return Some(Step::One(Beam {
                r: rr,
                c: cc,
                d: new_dir,
            }));
        }
        if let Some((a, b)) = self.d.split(&next_tile) {
            return Some(Step::Two(
                Beam { r: rr, c: cc, d: a },
                Beam { r: rr, c: cc, d: b },
            ));
        }
        None
    }
}

pub fn part1() -> Option<()> {
    let grid = parse(DATA);
    let result = find_energy(&grid, (1, 0, Dir::Right));
    println!("result = {result}");
    Some(())
}

pub fn part2() -> Option<()> {
    let grid = parse(DATA);
    let mut starts = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();
    for r in 0..rows {
        starts.push((r + 1, 0, Dir::Right));
        starts.push((r + 1, cols + 1, Dir::Left));
    }
    for c in 0..cols {
        starts.push((0, c + 1, Dir::Down));
        starts.push((rows + 1, c + 1, Dir::Up));
    }
    let mut results = Vec::with_capacity(starts.len());

    let (tx, rx) = mpsc::channel();
    let num_threads = std::thread::available_parallelism().ok()?.get();
    println!("Running with {num_threads} threads");
    thread::scope(|s| {
        let grid = &grid;
        let starts = &starts;
        for i in 0..num_threads {
            let tx1 = tx.clone();
            s.spawn(move || {
                for s in (i..starts.len()).step_by(num_threads) {
                    let start = starts[s];
                    let result = find_energy(&grid, start);
                    tx1.send(result).unwrap();
                }
            });
        }
    });
    drop(tx);

    while let Ok(v) = rx.recv() {
        results.push(v);
    }
    let result = results.into_iter().max()?;
    println!("result = {result}");
    Some(())
}

impl FromIterator<Step> for HashSet<Beam> {
    fn from_iter<I: IntoIterator<Item = Step>>(iter: I) -> Self {
        let mut s = HashSet::new();

        for step in iter {
            match step {
                Step::One(b) => {
                    s.insert(b);
                }
                Step::Two(a, b) => {
                    s.insert(a);
                    s.insert(b);
                }
            }
        }

        s
    }
}

fn find_energy(grid: &Grid, start: (usize, usize, Dir)) -> usize {
    let mut visited = HashSet::new();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut beams = HashSet::new();
    beams.insert(Beam {
        r: start.0,
        c: start.1,
        d: start.2,
    });
    let mut no_change_counter = 0;
    while !beams.is_empty() {
        beams = beams
            .into_iter()
            .flat_map(|b| b.step(&grid, rows, cols))
            .collect();
        let prev = visited.len();
        visited.extend(beams.iter().map(|b| (b.r, b.c)));
        let new = visited.len();
        if prev == new {
            no_change_counter += 1;
        } else {
            no_change_counter = 0;
        }
        if no_change_counter > 10 {
            break;
        }
    }
    visited.len()
}

fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '/' => Tile::MirrorBack,
                    '\\' => Tile::MirrorForward,
                    '|' => Tile::SplitterV,
                    '-' => Tile::SplitterH,
                    _ => panic!("bad input: {c}"),
                })
                .collect()
        })
        .collect()
}
