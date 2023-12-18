use std::collections::{BinaryHeap, HashSet};

const DATA: &str = include_str!("day17.txt");

type Grid = Vec<Vec<u32>>;

pub fn part1() -> Option<()> {
    let grid = parse(DATA);
    let result = least_energy(&grid, 1, 3);
    println!("result = {result}");
    Some(())
}

pub fn part2() -> Option<()> {
    let grid = parse(DATA);
    let result = least_energy(&grid, 4, 10);
    println!("result = {result}");
    Some(())
}

fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

use Dir::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    r: usize,
    c: usize,
    dir: Dir,
    dist: u32,
    steps: usize,
}

impl PartialOrd<Pos> for Pos {
    fn partial_cmp(&self, other: &Pos) -> Option<std::cmp::Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Pos) -> std::cmp::Ordering {
        other.dist.partial_cmp(&self.dist).unwrap()
    }
}

impl Pos {
    fn is(&self, (r, c): (usize, usize)) -> bool {
        self.r == r && self.c == c
    }

    fn key(&self) -> (usize, usize, Dir, usize) {
        (self.r, self.c, self.dir, self.steps)
    }

    fn neighbors(&self, grid: &Grid, min: usize, max: usize) -> Vec<Pos> {
        let mut n = Vec::new();
        let rows = grid.len();
        let cols = grid[0].len();
        match self.dir {
            Left | Right => {
                let mut dist = 0;
                for r in 1..=max {
                    if self.r < r {
                        break;
                    }
                    dist += grid[self.r - r][self.c];
                    if r >= min {
                        n.push(Pos {
                            r: self.r - r,
                            c: self.c,
                            dir: Up,
                            dist,
                            steps: r,
                        });
                    }
                }
                dist = 0;
                for r in 1..=max {
                    if self.r + r >= rows {
                        break;
                    }
                    dist += grid[self.r + r][self.c];
                    if r >= min {
                        n.push(Pos {
                            r: self.r + r,
                            c: self.c,
                            dir: Down,
                            dist,
                            steps: r,
                        });
                    }
                }
            }
            Up | Down => {
                let mut dist = 0;
                for c in 1..=max {
                    if self.c < c {
                        break;
                    }
                    dist += grid[self.r][self.c - c];
                    if c >= min {
                        n.push(Pos {
                            r: self.r,
                            c: self.c - c,
                            dir: Left,
                            dist,
                            steps: c,
                        });
                    }
                }
                dist = 0;
                for c in 1..=max {
                    if self.c + c >= cols {
                        break;
                    }
                    dist += grid[self.r][self.c + c];
                    if c >= min {
                        n.push(Pos {
                            r: self.r,
                            c: self.c + c,
                            dir: Right,
                            dist,
                            steps: c,
                        });
                    }
                }
            }
        }
        n
    }
}

fn initial_positions(grid: &Grid, min: usize, max: usize) -> Vec<Pos> {
    let mut result = Vec::new();
    let mut dist = 0;
    for r in 1..=max {
        dist += grid[r][0];
        if r >= min {
            let pos = Pos {
                r,
                c: 0,
                dir: Down,
                dist,
                steps: r,
            };
            result.push(pos);
        }
    }
    dist = 0;
    for c in 1..=max {
        dist += grid[0][c];
        if c >= min {
            let pos = Pos {
                r: 0,
                c,
                dir: Right,
                dist,
                steps: c,
            };
            result.push(pos);
        }
    }
    result
}

fn least_energy(grid: &Grid, min: usize, max: usize) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let target = (rows - 1, cols - 1);

    let mut q = BinaryHeap::from(initial_positions(grid, min, max));
    let mut visited = HashSet::new();

    while !q.is_empty() {
        let curr = q.pop().unwrap();

        if !visited.insert(curr.key()) {
            continue;
        }

        if curr.is(target) {
            return curr.dist;
        }

        for n in curr.neighbors(grid, min, max) {
            let dist = curr.dist + n.dist;
            q.push(Pos { dist, ..n });
        }
    }

    unreachable!()
}
