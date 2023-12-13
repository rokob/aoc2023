const DATA: &str = include_str!("day13.txt");

pub fn part1() -> Option<()> {
    let grids = parse(DATA);
    let mut result = 0;
    for grid in grids.into_iter() {
        let (r, c) = find_symmetry(&grid)?;
        result += 100 * r + c;
    }
    println!("result = {result}");
    Some(())
}

pub fn part2() -> Option<()> {
    let grids = parse(DATA);

    let mut result = 0;
    for grid in grids.into_iter() {
        let (r, c) = find_flipped_symmetry(&grid)?;
        result += 100 * r + c;
    }
    println!("result = {result}");
    Some(())
}

fn parse(input: &str) -> Vec<Vec<Vec<usize>>> {
    let mut grids = Vec::new();
    let mut curr = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            grids.push(curr);
            curr = Vec::new();
            continue;
        }
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(if c == '#' { 1usize } else { 0 });
        }
        curr.push(row);
    }
    grids.push(curr);

    grids
}

fn compute_row_hash(s: &Vec<Vec<usize>>, row: usize) -> usize {
    let mut h = 0;
    for v in s[row].iter() {
        h = h << 1 | *v;
    }
    h
}

fn flip_hash_at_index(h: usize, index: usize) -> usize {
    h ^ (1 << index)
}

fn compute_col_hash(s: &Vec<Vec<usize>>, col: usize) -> usize {
    let mut h = 0;
    for row in s.iter() {
        let v = row[col];
        h = h << 1 | v;
    }
    h
}

fn find_hash_symmetry(hashes: &Vec<usize>, ignoring: Option<usize>) -> Option<usize> {
    for i in 1..hashes.len() {
        if Some(i) == ignoring {
            continue;
        }
        let mut j = 1;
        loop {
            if hashes[i - j] != hashes[i + j - 1] {
                break;
            }
            if i - j == 0 || i + j - 1 == hashes.len() - 1 {
                return Some(i);
            }
            j += 1;
        }
    }
    None
}

fn find_symmetry(grid: &Vec<Vec<usize>>) -> Option<(usize, usize)> {
    let row_hash = (0..grid.len())
        .map(|row| compute_row_hash(grid, row))
        .collect();
    if let Some(i) = find_hash_symmetry(&row_hash, None) {
        return Some((i, 0));
    }
    let col_hash = (0..grid[0].len())
        .map(|col| compute_col_hash(grid, col))
        .collect();
    if let Some(i) = find_hash_symmetry(&col_hash, None) {
        return Some((0, i));
    }
    None
}

fn find_flipped_symmetry(grid: &Vec<Vec<usize>>) -> Option<(usize, usize)> {
    let unflipped = find_symmetry(&grid)?;

    let mut row_hash: Vec<usize> = (0..grid.len())
        .map(|row| compute_row_hash(grid, row))
        .collect();
    let mut col_hash: Vec<usize> = (0..grid[0].len())
        .map(|col| compute_col_hash(grid, col))
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    for r in 0..rows {
        for c in 0..cols {
            row_hash[r] = flip_hash_at_index(row_hash[r], cols - c - 1);
            if let Some(v) = find_hash_symmetry(&row_hash, Some(unflipped.0)) {
                return Some((v, 0));
            }
            row_hash[r] = flip_hash_at_index(row_hash[r], cols - c - 1);

            col_hash[c] = flip_hash_at_index(col_hash[c], rows - r - 1);
            if let Some(v) = find_hash_symmetry(&col_hash, Some(unflipped.1)) {
                return Some((0, v));
            }
            col_hash[c] = flip_hash_at_index(col_hash[c], rows - r - 1);
        }
    }
    None
}
