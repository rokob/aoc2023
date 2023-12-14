use std::collections::HashMap;

const DATA: &str = include_str!("day14.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Thing {
    Round,
    Cube,
    Empty,
}

type Grid = Vec<Vec<Thing>>;

pub fn part1() -> Option<()> {
    let mut grid = parse(DATA);
    tilt_vertical(&mut grid, true);
    let result = score(&grid);
    println!("result = {result}");
    Some(())
}

pub fn part2() -> Option<()> {
    let total_cycles = 1000000000;
    let grid = parse(DATA);
    let grid = tilt(grid, total_cycles)?;
    let result = score(&grid);
    println!("result = {result}");
    Some(())
}

fn parse(input: &str) -> Grid {
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(match c {
                '.' => Thing::Empty,
                '#' => Thing::Cube,
                'O' => Thing::Round,
                _ => panic!("bad times: {c}"),
            });
        }
        grid.push(row);
    }
    grid
}

fn tilt(mut grid: Grid, total_cycles: usize) -> Option<Grid> {
    let mut seen = HashMap::new();
    let mut counter = 0;
    while counter < total_cycles {
        tilt_once(&mut grid);
        counter += 1;
        if seen.contains_key(&grid) {
            break;
        }
        seen.insert(grid.clone(), counter);
    }
    if counter < total_cycles {
        let cycle_start = seen.get(&grid)?;
        let cycle_length = counter - cycle_start;
        let rem = (total_cycles - counter) % cycle_length;
        for _ in 0..rem {
            tilt_once(&mut grid);
        }
    }
    Some(grid)
}

fn tilt_once(grid: &mut Grid) {
    tilt_vertical(grid, true);
    tilt_horizontal(grid, true);
    tilt_vertical(grid, false);
    tilt_horizontal(grid, false);
}

fn tilt_vertical(grid: &mut Grid, is_north: bool) {
    let rows = grid.len();
    let cols = grid[0].len();
    let (offset, edge) = if is_north {
        (1i32, 0usize)
    } else {
        (-1 as i32, rows - 1 as usize)
    };
    for c in 0..cols {
        let mut boundary = None;
        for rr in 0..rows {
            let r = if is_north { rr } else { rows - rr - 1 };

            match grid[r][c] {
                Thing::Empty => {}
                Thing::Cube => {
                    boundary = Some(r);
                }
                Thing::Round => {
                    if let Some(k) = boundary {
                        let z = (k as i32 + offset) as usize;
                        if z != r {
                            grid[z][c] = Thing::Round;
                            grid[r][c] = Thing::Empty;
                        }
                        boundary = Some(z);
                    } else if r != edge {
                        grid[edge][c] = Thing::Round;
                        grid[r][c] = Thing::Empty;
                        boundary = Some(edge);
                    } else {
                        boundary = Some(r);
                    }
                }
            }
        }
    }
}

fn tilt_horizontal(grid: &mut Grid, is_west: bool) {
    let rows = grid.len();
    let cols = grid[0].len();
    let (offset, edge) = if is_west {
        (1i32, 0usize)
    } else {
        (-1 as i32, cols - 1 as usize)
    };
    for r in 0..rows {
        let mut boundary = None;
        for cc in 0..cols {
            let c = if is_west { cc } else { cols - cc - 1 };
            match grid[r][c] {
                Thing::Empty => {}
                Thing::Cube => {
                    boundary = Some(c);
                }
                Thing::Round => {
                    if let Some(k) = boundary {
                        let z = (k as i32 + offset) as usize;
                        if z != c {
                            grid[r][z] = Thing::Round;
                            grid[r][c] = Thing::Empty;
                        }
                        boundary = Some(z);
                    } else if c != edge {
                        grid[r][edge] = Thing::Round;
                        grid[r][c] = Thing::Empty;
                        boundary = Some(edge);
                    } else {
                        boundary = Some(c);
                    }
                }
            }
        }
    }
}

fn score(grid: &Grid) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut result = 0;
    for r in 0..rows {
        let factor = rows - r;
        for c in 0..cols {
            if grid[r][c] == Thing::Round {
                result += factor;
            }
        }
    }
    result
}
