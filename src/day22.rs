use std::collections::{HashMap, HashSet, VecDeque};

const DATA: &str = include_str!("day22.txt");

pub fn part1() -> Option<()> {
    let blocks = parse(DATA);
    let mut range = Range::new();
    for b in blocks.0.iter() {
        range.update(b);
    }
    let mut grid = Grid::new(blocks, range);
    grid.settle();

    let result = grid.count_disintegrated();
    println!("result = {result}");
    Some(())
}

pub fn part2() -> Option<()> {
    let blocks = parse(DATA);
    let mut range = Range::new();
    for b in blocks.0.iter() {
        range.update(b);
    }
    let mut grid = Grid::new(blocks, range);
    grid.settle();

    let result = grid.count_chain_reactions();
    println!("result = {result}");
    Some(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    range: Range,
    grid: Vec<Vec<Vec<u32>>>,
    blocks: Blocks,
    supports: HashMap<u32, Vec<u32>>,
    supported: HashMap<u32, Vec<u32>>,
}

impl Grid {
    fn count_chain_reactions(&self) -> usize {
        let mut result = 0;
        for b in self.blocks.0.iter() {
            result += self.chain_reactions_for(b.id);
        }
        result
    }

    fn chain_reactions_for(&self, id: u32) -> usize {
        let mut fallen = HashSet::new();
        let mut gonzo = VecDeque::new();
        gonzo.push_back(id);
        fallen.insert(id);
        while !gonzo.is_empty() {
            let v = gonzo.pop_front().unwrap();
            if let Some(mv) = self.supports.get(&v) {
                for m in mv.iter() {
                    let se = self.supported.get(m).unwrap();
                    if se.iter().all(|v| fallen.contains(v)) {
                        fallen.insert(*m);
                        gonzo.push_back(*m);
                    }
                }
            }
        }
        // we subtract one because we don't count the initial block itself
        fallen.len() - 1
    }

    fn count_disintegrated(&self) -> usize {
        let mut not_disintegrate = HashSet::new();
        for (_, v) in self.supported.iter() {
            if v.len() == 1 {
                let x = v[0];
                if x != 0 {
                    not_disintegrate.insert(x);
                }
            }
        }
        self.blocks.0.len() - not_disintegrate.len()
    }

    fn new(blocks: Blocks, range: Range) -> Self {
        let mut grid = Vec::new();
        for _ in 0..=range.z[1] {
            let mut r = Vec::new();
            for _ in range.x[0]..=range.x[1] {
                let mut c = Vec::new();
                for _ in range.y[0]..=range.y[1] {
                    c.push(0);
                }
                r.push(c);
            }
            grid.push(r);
        }

        for b in blocks.0.iter() {
            for z in b.start.z..=b.end.z {
                for x in b.start.x..=b.end.x {
                    for y in b.start.y..=b.end.y {
                        grid[z as usize][x as usize][y as usize] = b.id;
                    }
                }
            }
        }
        Grid {
            blocks,
            range,
            grid,
            supports: HashMap::new(),
            supported: HashMap::new(),
        }
    }

    fn settle(&mut self) {
        for x in self.range.x[0]..=self.range.x[1] {
            for y in self.range.y[0]..=self.range.y[1] {
                let b = self.grid[1][x as usize][y as usize];
                if b != 0 && !self.supported.contains_key(&b) {
                    self.supported.insert(b, vec![0]);
                    let e = self.supports.entry(0).or_insert_with(|| Vec::new());
                    e.push(b);
                }
            }
        }
        for z in 2..=self.range.z[1] {
            for x in self.range.x[0]..=self.range.x[1] {
                for y in self.range.y[0]..=self.range.y[1] {
                    let b = self.grid[z as usize][x as usize][y as usize];
                    if b == 0 || self.supported.contains_key(&b) {
                        continue;
                    }
                    let mut zz = z - 1;
                    let bb = &self.blocks.0[b as usize - 1];
                    while zz > 0 {
                        for xx in bb.start.x..=bb.end.x {
                            for yy in bb.start.y..=bb.end.y {
                                let sb = self.grid[zz as usize][xx as usize][yy as usize];
                                if sb != 0 {
                                    let spe = self.supported.entry(b).or_insert_with(|| Vec::new());
                                    if !spe.contains(&sb) {
                                        spe.push(sb);
                                    }
                                    let se = self.supports.entry(sb).or_insert_with(|| Vec::new());
                                    if !se.contains(&b) {
                                        se.push(b);
                                    }
                                }
                            }
                        }
                        if self.supported.contains_key(&b) {
                            break;
                        }
                        let offset = 1 + bb.end.z - bb.start.z;
                        for xx in bb.start.x..=bb.end.x {
                            for yy in bb.start.y..=bb.end.y {
                                self.grid[(zz + offset) as usize][xx as usize][yy as usize] = 0;
                                self.grid[zz as usize][xx as usize][yy as usize] = b;
                            }
                        }
                        zz -= 1;
                    }
                    if !self.supported.contains_key(&b) {
                        let spe = self.supported.entry(b).or_insert_with(|| Vec::new());
                        if !spe.contains(&0) {
                            spe.push(0);
                        }
                        let se = self.supports.entry(0).or_insert_with(|| Vec::new());
                        if !se.contains(&b) {
                            se.push(b);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Range {
    x: [u8; 2],
    y: [u8; 2],
    z: [u16; 2],
}

impl Range {
    fn new() -> Self {
        Range {
            x: [std::u8::MAX, std::u8::MIN],
            y: [std::u8::MAX, std::u8::MIN],
            z: [std::u16::MAX, std::u16::MIN],
        }
    }

    fn update(&mut self, b: &Block) {
        let x = [
            std::cmp::min(b.start.x, b.end.x),
            std::cmp::max(b.start.x, b.end.x),
        ];
        let y = [
            std::cmp::min(b.start.y, b.end.y),
            std::cmp::max(b.start.y, b.end.y),
        ];
        let z = [
            std::cmp::min(b.start.z, b.end.z),
            std::cmp::max(b.start.z, b.end.z),
        ];
        self.x[0] = std::cmp::min(self.x[0], x[0]);
        self.x[1] = std::cmp::max(self.x[1], x[1]);
        self.y[0] = std::cmp::min(self.y[0], y[0]);
        self.y[1] = std::cmp::max(self.y[1], y[1]);
        self.z[0] = std::cmp::min(self.z[0], z[0]);
        self.z[1] = std::cmp::max(self.z[1], z[1]);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pos {
    x: u8,
    y: u8,
    z: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Block {
    id: u32,
    start: Pos,
    end: Pos,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Blocks(Vec<Block>);

impl FromIterator<u16> for Pos {
    fn from_iter<T: IntoIterator<Item = u16>>(iter: T) -> Self {
        let mut i = iter.into_iter();
        let (x, y, z) = (i.next().unwrap(), i.next().unwrap(), i.next().unwrap());
        Pos {
            x: x as u8,
            y: y as u8,
            z,
        }
    }
}

fn parse(input: &str) -> Blocks {
    let mut blocks = Vec::new();
    for line in input.lines() {
        let (start, end) = line.split_once("~").unwrap();
        let start = start
            .split(",")
            .map(|v| v.parse::<u16>().unwrap())
            .collect::<Pos>();
        let end = end
            .split(",")
            .map(|v| v.parse::<u16>().unwrap())
            .collect::<Pos>();

        let id = (blocks.len() + 1) as u32;
        blocks.push(Block { id, start, end });
    }
    Blocks(blocks)
}
