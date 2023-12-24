use std::collections::HashSet;
const DATA: &str = include_str!("day21.txt");

pub fn part1() -> Option<()> {
    let (grid, start) = parse(DATA);

    let result = solve(&grid, start, 64);

    println!("result = {result}");
    Some(())
}

fn solve(grid: &Grid, start: Pos, steps: usize) -> usize {
    let mut curr = HashSet::new();
    curr.insert(start);
    for _ in 1..=steps {
        let mut next = HashSet::new();
        for n in curr.into_iter() {
            for adj in neighbors(&grid, n) {
                next.insert(adj);
            }
        }
        curr = next;
    }

    curr.len()
}

fn isolve(grid: &Grid, start: Pos, steps: usize) -> usize {
    let mut curr = HashSet::new();
    curr.insert(((0, 0), start));
    for _ in 1..=steps {
        let mut next = HashSet::new();
        for n in curr.into_iter() {
            for adj in ineighbors(&grid, n) {
                next.insert(adj);
            }
        }
        curr = next;
    }

    curr.len()
}

pub fn part2() -> Option<()> {
    let (grid, start) = parse(DATA);

    let goal = 26501365;
    let rows = grid.len();
    let a = goal % rows;

    let b0 = isolve(&grid, start, a) as isize;
    let b1 = isolve(&grid, start, a + rows) as isize;
    let b2 = isolve(&grid, start, a + rows + rows) as isize;

    let n = ((goal - a) / rows) as isize;
    /*
     * Lagrange...
    b0 (n - 1)*(n-2) / (-1*-2) = b0*(n^2 - 3n + 2) / 2
    b1 n*(n-2)/(-1) = -b1*(n^2 - 2n)
    b2 n*(n-1)/2 = b2*(n^2 - n) / 2

    => z0 = (b0 - 2* b1 + b2)/2
       z1 = -3*b0/2 + 2*b1 - b2/2 = b1 - b0 - (b0 / 2) + b1 - b2/2 = b1 - b0 - z0
       z2 = b0
    */
    let z0 = (b0 - 2 * b1 + b2) / 2;
    let z1 = b1 - b0 - z0;
    let z2 = b0;

    let result = z2 + z1 * n + z0 * n * n;
    println!("result = {result}");

    Some(())
}

fn neighbors(grid: &Grid, pos: Pos) -> Vec<Pos> {
    let mut result = Vec::new();
    if pos.0 > 0 && is_garden(grid, (pos.0 - 1, pos.1)) {
        result.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 && is_garden(grid, (pos.0, pos.1 - 1)) {
        result.push((pos.0, pos.1 - 1));
    }
    if pos.0 < grid.len() - 1 && is_garden(grid, (pos.0 + 1, pos.1)) {
        result.push((pos.0 + 1, pos.1));
    }
    if pos.1 < grid[0].len() - 1 && is_garden(grid, (pos.0, pos.1 + 1)) {
        result.push((pos.0, pos.1 + 1));
    }
    result
}

fn ineighbors(grid: &Grid, pos: IPos) -> Vec<IPos> {
    let mut result = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();
    let north = if pos.1 .0 > 0 {
        (pos.0, (pos.1 .0 - 1, pos.1 .1))
    } else {
        ((pos.0 .0 - 1, pos.0 .1), (rows - 1, pos.1 .1))
    };
    let south = if pos.1 .0 < rows - 1 {
        (pos.0, (pos.1 .0 + 1, pos.1 .1))
    } else {
        ((pos.0 .0 + 1, pos.0 .1), (0, pos.1 .1))
    };
    let west = if pos.1 .1 > 0 {
        (pos.0, (pos.1 .0, pos.1 .1 - 1))
    } else {
        ((pos.0 .0, pos.0 .1 - 1), (pos.1 .0, cols - 1))
    };
    let east = if pos.1 .1 < cols - 1 {
        (pos.0, (pos.1 .0, pos.1 .1 + 1))
    } else {
        ((pos.0 .0, pos.0 .1 + 1), (pos.1 .0, 0))
    };
    for p in [north, south, east, west] {
        if is_garden(grid, p.1) {
            result.push(p);
        }
    }
    result
}

fn is_garden(grid: &Grid, pos: Pos) -> bool {
    match grid[pos.0][pos.1] {
        Tile::Rock => false,
        Tile::Garden => true,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Garden,
    Rock,
}

type Pos = (usize, usize);
type IPos = ((i32, i32), Pos);
type Grid = Vec<Vec<Tile>>;

fn parse(input: &str) -> (Grid, Pos) {
    let mut start: (usize, usize) = (0, 0);
    let g = input
        .lines()
        .enumerate()
        .map(|(r, row)| {
            row.chars()
                .enumerate()
                .map(|(c, v)| {
                    if v == '#' {
                        Tile::Rock
                    } else {
                        if v == 'S' {
                            start = (r, c);
                        }
                        Tile::Garden
                    }
                })
                .collect()
        })
        .collect();
    (g, start)
}

fn print_grid(grid: &Grid, pts: &HashSet<Pos>) {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if pts.contains(&(r, c)) {
                print!("O");
            } else if grid[r][c] == Tile::Rock {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn iprint_grid(grid: &Grid, pts: &HashSet<IPos>) {
    for rr in -1..=1 {
        for r in 0..grid.len() {
            for cc in -1..=1 {
                for c in 0..grid[0].len() {
                    if pts.contains(&((rr, cc), (r, c))) {
                        print!("O");
                    } else if grid[r][c] == Tile::Rock {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
            }
            println!("");
        }
    }
}
