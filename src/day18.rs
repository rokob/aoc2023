use std::collections::HashMap;

const DATA: &str = include_str!("day18.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Tile {
    interior: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    r: isize,
    c: isize,
}

type Grid = HashMap<Pos, Tile>;

pub fn part1() -> Option<()> {
    let (mut grid, min, max) = parse(DATA);
    fill_interior(&mut grid, min, max);
    let result = grid.len();
    println!("result = {result}");
    Some(())
}

pub fn part2() -> Option<()> {
    let (points, boundary_size) = parse_color(DATA);
    let area = shoelace_area(&points);
    let interior = area - boundary_size / 2 + 1;
    let result = interior + boundary_size;
    println!("result = {result}");
    Some(())
}

fn shoelace_area(points: &Vec<Pos>) -> isize {
    let mut area = 0isize;
    for point in points.windows(3) {
        let (pim, pi, pip) = (point[0], point[1], point[2]);
        area += pi.r * (pip.c - pim.c);
    }
    let (pim, pi, pip) = (points[points.len() - 1], points[0], points[1]);
    area += pi.r * (pip.c - pim.c);
    let (pim, pi, pip) = (
        points[points.len() - 2],
        points[points.len() - 1],
        points[0],
    );
    area += pi.r * (pip.c - pim.c);
    (area / 2).abs()
}

fn parse_color(input: &str) -> (Vec<Pos>, isize) {
    let mut points = Vec::new();
    let mut cur = (0, 0);
    let mut boundary_size = 0;
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let _ = parts.next();
        let _ = parts.next();
        let color = parts
            .next()
            .unwrap()
            .trim_start_matches("(")
            .trim_end_matches(")");
        let digits = color.trim_start_matches("#");
        let count = digits.chars().take(5).collect::<String>();
        let count = isize::from_str_radix(&count, 16).unwrap();
        let dir = match digits.chars().last().unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => panic!("bad dir"),
        };
        let offset = match dir {
            'L' => (0, -1),
            'R' => (0, 1),
            'U' => (-1, 0),
            'D' => (1, 0),
            _ => panic!("bad dir: {dir}"),
        };
        boundary_size += count;
        cur = (cur.0 + count * offset.0, cur.1 + count * offset.1);
        points.push(Pos { r: cur.0, c: cur.1 });
    }
    (points, boundary_size)
}

fn parse(input: &str) -> (Grid, Pos, Pos) {
    let mut g = HashMap::new();
    let mut cur = (0, 0);
    let mut min_r = std::isize::MAX;
    let mut max_r = std::isize::MIN;
    let mut min_c = std::isize::MAX;
    let mut max_c = std::isize::MIN;
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let dir = parts.next().unwrap().chars().next().unwrap();
        let count = parts.next().unwrap().parse::<isize>().unwrap();
        let offset = match dir {
            'L' => (0, -1),
            'R' => (0, 1),
            'U' => (-1, 0),
            'D' => (1, 0),
            _ => panic!("bad dir: {dir}"),
        };
        for _ in 0..count {
            cur = (cur.0 + offset.0, cur.1 + offset.1);
            if cur.0 < min_r {
                min_r = cur.0;
            }
            if cur.0 > max_r {
                max_r = cur.0;
            }
            if cur.1 < min_c {
                min_c = cur.1;
            }
            if cur.1 > max_c {
                max_c = cur.1;
            }
            g.insert(Pos { r: cur.0, c: cur.1 }, Tile { interior: false });
        }
    }
    (g, Pos { r: min_r, c: min_c }, Pos { r: max_r, c: max_c })
}

fn print_grid(grid: &mut Grid, min: Pos, max: Pos) {
    for r in min.r..=max.r {
        for c in min.c..=max.c {
            if grid.contains_key(&Pos { r, c }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn fill_interior(grid: &mut Grid, min: Pos, max: Pos) {
    for r in min.r..=max.r {
        let mut on_edge = false;
        let mut inside = false;
        for c in min.c..=max.c {
            if let Some(Tile { interior, .. }) = grid.get(&Pos { r, c }) {
                if on_edge {
                    continue;
                }
                on_edge = !interior;
            } else {
                if on_edge && inside {
                    on_edge = false;
                    if let Some(Tile { interior, .. }) = grid.get(&Pos { r: r - 1, c }) {
                        if *interior {
                            grid.insert(Pos { r, c }, Tile { interior: true });
                        } else {
                            inside = false;
                        }
                    } else {
                        inside = false;
                    }
                } else if on_edge {
                    if !grid.contains_key(&Pos { r: r - 1, c }) {
                        continue;
                    }
                    on_edge = false;
                    inside = true;
                    grid.insert(Pos { r, c }, Tile { interior: true });
                } else if inside {
                    grid.insert(Pos { r, c }, Tile { interior: true });
                }
            }
        }
    }
}
