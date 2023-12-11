use std::collections::{HashMap, HashSet, VecDeque};
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
    let pipe_loop = get_loop(start_pos, start_kind, &grid)?;
    let mut result = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    print_loop(rows, cols, &pipe_loop, &grid);
    let new_loop = find_answer(rows, cols, &pipe_loop, &grid);
    println!("\n\n-----\n\n");
    print_loop(rows, cols, &new_loop, &grid);
    for r in 0..rows {
        for c in 0..cols {
            if !new_loop.contains_key(&(r, c)) {
                result += 1;
            }
        }
    }
    /*
    for r in 0..rows {
        for c in 0..cols {
            if pipe_loop.contains_key(&(r, c)) {
                continue;
            }
            let mut rr = r;
            let mut cc = c;
            let mut crossings = 0;
            while rr < rows && cc < cols {
                let c = grid[rr][cc];
                if pipe_loop.contains_key(&(rr, cc)) && c != Pipe::Ne && c != Pipe::Sw {
                    crossings += 1;
                }
                rr += 1;
                cc += 1;
            }
            if crossings % 2 == 1 {
                result += 1;
            }
        }
    }
    */
    println!("result = {result}");
    Some(())
}

fn find_answer(
    rows: usize,
    cols: usize,
    pipe_loop: &HashMap<(usize, usize), i32>,
    grid: &Vec<Vec<Pipe>>,
) -> HashMap<(usize, usize), i32> {
    let mut outside = HashSet::new();
    let mut helper = pipe_loop
        .iter()
        .map(|((r, c), _)| ((*r, *c), 1))
        .collect::<HashMap<(usize, usize), i32>>();
    let mut did_update = true;
    while did_update {
        did_update = false;
        for r in 0..rows {
            for c in 0..cols {
                if helper.contains_key(&(r, c)) || outside.contains(&(r, c)) {
                    continue;
                }
                let mut did_escape = false;
                let mut this_escape = true;
                for i in r + 1..rows {
                    if outside.contains(&(i, c)) {
                        this_escape = true;
                        break;
                    }
                    if helper.contains_key(&(i, c)) && is_blockage((i, c), &grid, Dir::S) {
                        this_escape = false;
                        break;
                    }
                }
                did_escape = did_escape || this_escape;
                if r > 0 {
                    this_escape = true;
                    for i in (0..r).rev() {
                        if outside.contains(&(i, c)) {
                            this_escape = true;
                            break;
                        }
                        if helper.contains_key(&(i, c)) && is_blockage((i, c), &grid, Dir::N) {
                            this_escape = false;
                            break;
                        }
                    }
                    did_escape = did_escape || this_escape;
                }
                this_escape = true;
                for j in c + 1..cols {
                    if outside.contains(&(r, j)) {
                        this_escape = true;
                        break;
                    }
                    if helper.contains_key(&(r, j)) && is_blockage((r, j), &grid, Dir::E) {
                        this_escape = false;
                        break;
                    }
                }
                did_escape = did_escape || this_escape;
                if c > 0 {
                    this_escape = true;
                    for j in (0..c).rev() {
                        if outside.contains(&(r, j)) {
                            this_escape = true;
                            break;
                        }
                        if helper.contains_key(&(r, j)) && is_blockage((r, j), &grid, Dir::W) {
                            this_escape = false;
                            break;
                        }
                    }
                    did_escape = did_escape || this_escape;
                }
                if did_escape {
                    outside.insert((r, c));
                    did_update = true;
                }
            }
        }
    }
    for (r, c) in outside.into_iter() {
        helper.insert((r, c), 0);
    }
    helper
}

fn is_blockage((r, c): (usize, usize), grid: &Vec<Vec<Pipe>>, dir: Dir) -> bool {
    match grid[r][c] {
        Pipe::Ne => {
            if let Some(col) = grid.get(r - 1) {
                if Some(&Pipe::Se) == col.get(c) {
                    return dir == Dir::E || dir == Dir::W;
                }
            }
            if let Some(col) = grid.get(r) {
                if Some(&Pipe::Nw) == col.get(c + 1) {
                    return dir == Dir::N || dir == Dir::S;
                }
            }
        }
        Pipe::Se => {
            if let Some(col) = grid.get(r + 1) {
                if Some(&Pipe::Ne) == col.get(c) {
                    return dir == Dir::E || dir == Dir::W;
                }
            }
            if let Some(col) = grid.get(r) {
                if Some(&Pipe::Sw) == col.get(c + 1) {
                    return dir == Dir::N || dir == Dir::S;
                }
            }
        }
        Pipe::Nw => {
            if let Some(col) = grid.get(r - 1) {
                if Some(&Pipe::Sw) == col.get(c) {
                    return dir == Dir::E || dir == Dir::W;
                }
            }
            if let Some(col) = grid.get(r) {
                if Some(&Pipe::Ne) == col.get(c - 1) {
                    return dir == Dir::N || dir == Dir::S;
                }
            }
        }
        Pipe::Sw | Pipe::Start => {
            if let Some(col) = grid.get(r + 1) {
                if Some(&Pipe::Nw) == col.get(c) {
                    return dir == Dir::E || dir == Dir::W;
                }
            }
            if let Some(col) = grid.get(r) {
                if Some(&Pipe::Se) == col.get(c - 1) {
                    return dir == Dir::N || dir == Dir::S;
                }
            }
        }
        Pipe::Ew => return true,
        Pipe::Ns => return true,
        Pipe::None => return false,
    }
    false
}

fn print_loop(
    rows: usize,
    cols: usize,
    pipe_loop: &HashMap<(usize, usize), i32>,
    grid: &Vec<Vec<Pipe>>,
) {
    for r in 0..rows {
        for c in 0..cols {
            match pipe_loop.get(&(r, c)) {
                Some(0) => print!(" "),
                Some(_) => {
                    let mut did_print = false;
                    match grid[r][c] {
                        Pipe::Ne => {
                            if let Some(col) = grid.get(r - 1) {
                                if Some(&Pipe::Se) == col.get(c) {
                                    print!("L");
                                    did_print = true;
                                }
                            }
                            if !did_print {
                                if let Some(col) = grid.get(r) {
                                    if Some(&Pipe::Nw) == col.get(c + 1) {
                                        print!("L");
                                        did_print = true;
                                    }
                                }
                            }
                        }
                        Pipe::Se => {
                            if let Some(col) = grid.get(r + 1) {
                                if Some(&Pipe::Ne) == col.get(c) {
                                    print!("F");
                                    did_print = true;
                                }
                            }
                            if !did_print {
                                if let Some(col) = grid.get(r) {
                                    if Some(&Pipe::Sw) == col.get(c + 1) {
                                        print!("F");
                                        did_print = true;
                                    }
                                }
                            }
                        }
                        Pipe::Nw => {
                            if let Some(col) = grid.get(r - 1) {
                                if Some(&Pipe::Sw) == col.get(c) {
                                    print!("J");
                                    did_print = true;
                                }
                            }
                            if !did_print {
                                if let Some(col) = grid.get(r) {
                                    if Some(&Pipe::Ne) == col.get(c - 1) {
                                        print!("J");
                                        did_print = true;
                                    }
                                }
                            }
                        }
                        Pipe::Sw => {
                            if let Some(col) = grid.get(r + 1) {
                                if Some(&Pipe::Nw) == col.get(c) {
                                    print!("7");
                                    did_print = true;
                                }
                            }
                            if !did_print {
                                if let Some(col) = grid.get(r) {
                                    if Some(&Pipe::Se) == col.get(c - 1) {
                                        print!("7");
                                        did_print = true;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    if !did_print {
                        print!("X");
                    }
                }
                None => print!("."),
            }
        }
        println!("");
    }
}

fn find_crossings(
    start: (usize, usize),
    rows: usize,
    cols: usize,
    dir: Dir,
    pipe_loop: &HashMap<(usize, usize), i32>,
    insides: &HashSet<(usize, usize)>,
) -> bool {
    let mut crossings = 0;
    let mut is_crossing = false;
    let (mut curr, stride, end) = match dir {
        Dir::N => ((start.0 - 1, start.1), (-1, 0), (0i32, 0i32)),
        Dir::S => (
            (start.0 + 1, start.1),
            (1, 0),
            ((rows - 1) as i32, (cols - 1) as i32),
        ),
        Dir::E => (
            (start.0, start.1 + 1),
            (0, 1),
            ((rows - 1) as i32, (cols - 1) as i32),
        ),
        Dir::W => ((start.0, start.1 - 1), (0, -1), (0i32, 0i32)),
    };
    loop {
        if pipe_loop.contains_key(&(curr.0, curr.1)) || insides.contains(&(curr.0, curr.1)) {
            return true;
            if !is_crossing {
                is_crossing = true;
            }
        } else {
            if is_crossing {
                crossings += 1;
            }
            is_crossing = false
        }
        if stride.0 < 0 || stride.1 < 0 {
            if curr.0 as i32 + stride.0 < end.0 || curr.1 as i32 + stride.1 < end.1 {
                break;
            }
        } else {
            if curr.0 as i32 + stride.0 > end.0 || curr.1 as i32 + stride.1 > end.1 {
                break;
            }
        }
        curr = (
            (curr.0 as i32 + stride.0) as usize,
            (curr.1 as i32 + stride.1) as usize,
        );
    }
    if is_crossing {
        crossings += 1;
    }
    crossings > 0 && crossings % 2 != 0
}

fn is_blocked(a: Pipe, b: Pipe, horizontal: bool) -> bool {
    match (a, b, horizontal) {
        (Pipe::Ns, _, true) => false,

        (Pipe::Ns, Pipe::Ns, false) => true,
        (Pipe::Ns, Pipe::Sw, false) => true,
        (Pipe::Ns, Pipe::Se, false) => true,
        (Pipe::Ns, _, false) => false,

        (Pipe::Ew, Pipe::Ew, true) => true,
        (Pipe::Ew, Pipe::Nw, true) => true,
        (Pipe::Ew, Pipe::Sw, true) => true,
        (Pipe::Ew, _, true) => false,

        (Pipe::Ew, _, false) => false,

        (Pipe::Ne, Pipe::Ew, true) => true,
        (Pipe::Ne, Pipe::Nw, true) => true,
        (Pipe::Ne, Pipe::Sw, true) => true,
        (Pipe::Ne, _, true) => false,

        (Pipe::Ne, Pipe::Sw, false) => true,
        (Pipe::Ne, Pipe::Se, false) => true,
        (Pipe::Ne, Pipe::Ns, false) => true,
        (Pipe::Ne, _, false) => false,

        //Nw,
        (Pipe::Nw, Pipe::Ew, true) => true,
        (Pipe::Nw, Pipe::Se, true) => true,
        (Pipe::Nw, Pipe::Ne, true) => true,
        (Pipe::Nw, _, true) => false,

        (Pipe::Nw, Pipe::Sw, false) => true,
        (Pipe::Nw, Pipe::Se, false) => true,
        (Pipe::Nw, Pipe::Ns, false) => true,
        (Pipe::Nw, _, false) => false,
        //Sw,
        (Pipe::Sw, Pipe::Ew, true) => true,
        (Pipe::Sw, Pipe::Se, true) => true,
        (Pipe::Sw, Pipe::Ne, true) => true,
        (Pipe::Sw, _, true) => false,

        (Pipe::Sw, Pipe::Nw, false) => true,
        (Pipe::Sw, Pipe::Ne, false) => true,
        (Pipe::Sw, Pipe::Ns, false) => true,
        (Pipe::Sw, _, false) => false,
        //Se,
        (Pipe::Se, Pipe::Ew, true) => true,
        (Pipe::Se, Pipe::Nw, true) => true,
        (Pipe::Se, Pipe::Sw, true) => true,
        (Pipe::Se, _, true) => false,

        (Pipe::Se, Pipe::Nw, false) => true,
        (Pipe::Se, Pipe::Ne, false) => true,
        (Pipe::Se, Pipe::Ns, false) => true,
        (Pipe::Se, _, false) => false,
        _ => false,
    }
}

fn can_escape(r: usize, c: usize, grid: &Vec<Vec<Pipe>>) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut offset = 0;
    let mut curr = (r, c);
    loop {
        if offset == 0 {
            let left = (curr.0 - 1, curr.1 - 1);
            let center = (curr.0 - 1, curr.1);
            let right = (curr.0 - 1, curr.1 + 1);
            let ll = grid[left.0][left.1];
            let cc = grid[center.0][center.1];
            let rr = grid[right.0][right.1];
            let lc = ll.can_connect(&cc, Dir::E).unwrap_or(false);
            let cr = cc.can_connect(&rr, Dir::E).unwrap_or(false);
            if lc && cr {
                break;
            }
            if lc {
                offset = 1;
            } else if cr {
                offset = -1;
            }
        } else if offset == 1 {
            let center = (curr.0 - 1, curr.1);
            let right = (curr.0 - 1, curr.1 + 1);
            let cc = grid[center.0][center.1];
            let rr = grid[right.0][right.1];
            let cr = cc.can_connect(&rr, Dir::E).unwrap_or(false);
            if cr {
                break;
            }
        } else if offset == -1 {
            let left = (curr.0 - 1, curr.1 - 1);
            let center = (curr.0 - 1, curr.1);
            let ll = grid[left.0][left.1];
            let cc = grid[center.0][center.1];
            let lc = ll.can_connect(&cc, Dir::E).unwrap_or(false);
            if lc {
                break;
            }
        }
        if curr.0 - 1 == 0 {
            return true;
        }
        curr = (curr.0 - 1, curr.1);
    }
    offset = 0;
    curr = (r, c);
    loop {
        if offset == 0 {
            let left = (curr.0 + 1, curr.1 - 1);
            let center = (curr.0 + 1, curr.1);
            let right = (curr.0 + 1, curr.1 + 1);
            let ll = grid[left.0][left.1];
            let cc = grid[center.0][center.1];
            let rr = grid[right.0][right.1];
            let lc = ll.can_connect(&cc, Dir::E).unwrap_or(false);
            let cr = cc.can_connect(&rr, Dir::E).unwrap_or(false);
            if lc && cr {
                break;
            }
            if lc {
                offset = 1;
            } else if cr {
                offset = -1;
            }
        } else if offset == 1 {
            let center = (curr.0 + 1, curr.1);
            let right = (curr.0 + 1, curr.1 + 1);
            let cc = grid[center.0][center.1];
            let rr = grid[right.0][right.1];
            let cr = cc.can_connect(&rr, Dir::E).unwrap_or(false);
            if cr {
                break;
            }
        } else if offset == -1 {
            let left = (curr.0 + 1, curr.1 - 1);
            let center = (curr.0 + 1, curr.1);
            let ll = grid[left.0][left.1];
            let cc = grid[center.0][center.1];
            let lc = ll.can_connect(&cc, Dir::E).unwrap_or(false);
            if lc {
                break;
            }
        }
        if curr.0 + 1 == rows - 1 {
            return true;
        }
        curr = (curr.0 + 1, curr.1);
    }
    offset = 0;
    curr = (r, c);
    loop {
        if offset == 0 {
            let left = (curr.0 - 1, curr.1 - 1);
            let center = (curr.0, curr.1 - 1);
            let right = (curr.0 + 1, curr.1 - 1);
            let ll = grid[left.0][left.1];
            let cc = grid[center.0][center.1];
            let rr = grid[right.0][right.1];
            let lc = ll.can_connect(&cc, Dir::S).unwrap_or(false);
            let cr = cc.can_connect(&rr, Dir::S).unwrap_or(false);
            if lc && cr {
                break;
            }
            if lc {
                offset = 1;
            } else if cr {
                offset = -1;
            }
        } else if offset == 1 {
            let center = (curr.0, curr.1 - 1);
            let right = (curr.0 + 1, curr.1 - 1);
            let cc = grid[center.0][center.1];
            let rr = grid[right.0][right.1];
            let cr = cc.can_connect(&rr, Dir::S).unwrap_or(false);
            if cr {
                break;
            }
        } else if offset == -1 {
            let left = (curr.0 - 1, curr.1 - 1);
            let center = (curr.0, curr.1 - 1);
            let ll = grid[left.0][left.1];
            let cc = grid[center.0][center.1];
            let lc = ll.can_connect(&cc, Dir::S).unwrap_or(false);
            if lc {
                break;
            }
        }
        if curr.1 - 1 == 0 {
            return true;
        }
        curr = (curr.0, curr.1 - 1);
    }
    offset = 0;
    curr = (r, c);
    loop {
        if offset == 0 {
            let left = (curr.0 - 1, curr.1 + 1);
            let center = (curr.0, curr.1 + 1);
            let right = (curr.0 + 1, curr.1 + 1);
            let ll = grid[left.0][left.1];
            let cc = grid[center.0][center.1];
            let rr = grid[right.0][right.1];
            let lc = ll.can_connect(&cc, Dir::S).unwrap_or(false);
            let cr = cc.can_connect(&rr, Dir::S).unwrap_or(false);
            if lc && cr {
                break;
            }
            if lc {
                offset = 1;
            } else if cr {
                offset = -1;
            }
        } else if offset == 1 {
            let center = (curr.0, curr.1 + 1);
            let right = (curr.0 + 1, curr.1 + 1);
            let cc = grid[center.0][center.1];
            let rr = grid[right.0][right.1];
            let cr = cc.can_connect(&rr, Dir::S).unwrap_or(false);
            if cr {
                break;
            }
        } else if offset == -1 {
            let left = (curr.0 - 1, curr.1 + 1);
            let center = (curr.0, curr.1 + 1);
            let ll = grid[left.0][left.1];
            let cc = grid[center.0][center.1];
            let lc = ll.can_connect(&cc, Dir::S).unwrap_or(false);
            if lc {
                break;
            }
        }
        if curr.1 + 1 == cols - 1 {
            return true;
        }
        curr = (curr.0, curr.1 + 1);
    }
    false
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
) -> Option<HashMap<(usize, usize), i32>> {
    let mut paths = HashMap::new();
    let mut queue = VecDeque::new();
    paths.insert(start, 0);
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
    Some(paths)
}
