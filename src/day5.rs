const DATA: &str = include_str!("day5.txt");

#[derive(Debug)]
struct Range {
    dest: usize,
    source: usize,
    len: usize,
}

impl Range {
    fn output(&self, value: usize) -> Option<usize> {
        if self.source <= value && (self.source + self.len) > value {
            Some(value - self.source + self.dest)
        } else {
            None
        }
    }
}

pub fn part1() -> Option<()> {
    let mut seeds: Vec<usize> = Vec::new();
    let mut maps = Vec::new();
    for line in DATA.lines() {
        if line.starts_with("seeds: ") {
            seeds = line
                .split_once(": ")?
                .1
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            continue;
        }
        if line.starts_with("seed-to-soil") {
            maps.push(Vec::new());
            continue;
        }
        if line.starts_with("soil-to-fertilizer") {
            maps.push(Vec::new());
            continue;
        }
        if line.starts_with("fertilizer-to-water") {
            maps.push(Vec::new());
            continue;
        }
        if line.starts_with("water-to-light") {
            maps.push(Vec::new());
            continue;
        }
        if line.starts_with("light-to-temperature") {
            maps.push(Vec::new());
            continue;
        }
        if line.starts_with("temperature-to-humidity") {
            maps.push(Vec::new());
            continue;
        }
        if line.starts_with("humidity-to-location") {
            maps.push(Vec::new());
            continue;
        }
        if line.is_empty() {
            continue;
        }

        let idx = maps.len() - 1;
        let parts = line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        maps[idx].push(Range {
            dest: parts[0],
            source: parts[1],
            len: parts[2],
        });
    }
    let result = seeds.iter().map(|s| map_through_range(*s, &maps)).min()?;
    println!("result = {result}");
    Some(())
}

#[derive(Debug, Copy, Clone)]
struct SeedRange {
    start: usize,
    len: usize,
}

pub fn part2() -> Option<()> {
    let mut seeds = Vec::new();
    let mut maps = Vec::new();
    for line in DATA.lines() {
        if line.starts_with("seeds: ") {
            let seed_data: Vec<_> = line
                .split_once(": ")?
                .1
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            seeds = seed_data
                .chunks(2)
                .map(|c| SeedRange {
                    start: c[0],
                    len: c[1],
                })
                .collect();

            continue;
        }
        if line.starts_with("seed-to-soil") {
            maps.push(Vec::new());
            continue;
        }
        if line.starts_with("soil-to-fertilizer") {
            maps.push(Vec::new());
            continue;
        }
        if line.starts_with("fertilizer-to-water") {
            maps.push(Vec::new());
            continue;
        }
        if line.starts_with("water-to-light") {
            maps.push(Vec::new());
            continue;
        }
        if line.starts_with("light-to-temperature") {
            maps.push(Vec::new());
            continue;
        }
        if line.starts_with("temperature-to-humidity") {
            maps.push(Vec::new());
            continue;
        }
        if line.starts_with("humidity-to-location") {
            maps.push(Vec::new());
            continue;
        }
        if line.is_empty() {
            continue;
        }

        let idx = maps.len() - 1;
        let parts = line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        maps[idx].push(Range {
            dest: parts[0],
            source: parts[1],
            len: parts[2],
        });
    }

    let mut result = std::usize::MAX;
    for sr in seeds.iter() {
        for s in sr.start..sr.start + sr.len {
            result = std::cmp::min(result, map_through_range(s, &maps));
        }
    }
    println!("result = {result}");
    Some(())
}

fn map_through_range(seed: usize, maps: &Vec<Vec<Range>>) -> usize {
    let mut curr = seed;
    for i in 0..maps.len() {
        let out = maps[i].iter().find_map(|r| r.output(curr));
        if let Some(v) = out {
            curr = v;
        }
    }
    return curr;
}

impl SeedRange {
    fn overlap(&self, other: &SeedRange) -> Option<usize> {
        if self.start < other.start + other.len && self.start + self.len > other.start {
            let start = if self.start <= other.start {
                other.start
            } else {
                self.start
            };
            Some(start)
        } else {
            None
        }
    }

    fn decompse(&self, start: usize, len: usize) -> Vec<Self> {
        let mut result = Vec::new();
        if start < self.start {
            result.push(SeedRange {
                start,
                len: self.start - start,
            });
        }
        result.push(SeedRange { start, len });
        if start + len < self.start + self.len {
            result.push(SeedRange {
                start: start + len,
                len: self.start + self.len - start + len,
            });
        }
        return result;
    }

    fn get_source_overlapping(
        &self,
        range: &Range,
    ) -> Option<(Option<SeedRange>, SeedRange, Option<SeedRange>)> {
        if self.start >= range.source && self.start + self.len <= range.source + range.len {
            Some((
                None,
                SeedRange {
                    start: self.start,
                    len: self.len,
                },
                None,
            ))
        } else if range.source >= self.start && range.source + range.len <= self.start + self.len {
            let start = if range.source > self.start {
                Some(SeedRange {
                    start: self.start,
                    len: range.source - self.start,
                })
            } else {
                None
            };
            let end = if self.start + self.len > range.source + range.len {
                Some(SeedRange {
                    start: range.source + range.len,
                    len: self.start + self.len - range.source - range.len,
                })
            } else {
                None
            };
            Some((
                start,
                SeedRange {
                    start: range.source,
                    len: range.len,
                },
                end,
            ))
        } else if self.start < range.source
            && self.start + self.len >= range.source
            && range.source + range.len > self.start + self.len
        {
            let start = if range.source > self.start {
                Some(SeedRange {
                    start: self.start,
                    len: range.source - self.start,
                })
            } else {
                None
            };
            let end = if self.start + self.len > range.len {
                Some(SeedRange {
                    start: range.source + range.source + range.len - self.start - self.len,
                    len: self.start + self.len - range.len,
                })
            } else {
                None
            };
            Some((
                start,
                SeedRange {
                    start: range.source,
                    len: range.source + range.len - self.start - self.len,
                },
                end,
            ))
        } else if range.source < self.start
            && range.source + range.len >= self.start
            && self.start + self.len > range.source + range.len
        {
            let start = if self.start > range.source {
                Some(SeedRange {
                    start: range.source,
                    len: self.start - range.source,
                })
            } else {
                None
            };
            let end = if range.source + range.len > self.len {
                Some(SeedRange {
                    start: self.start + self.start + self.len - range.source - range.len,
                    len: range.source + range.len - self.len,
                })
            } else {
                None
            };
            Some((
                start,
                SeedRange {
                    start: self.start,
                    len: self.start + self.len - range.source - range.len,
                },
                end,
            ))
        } else {
            None
        }
    }

    fn get_overlapping(&self, range: &Range) -> Option<(usize, usize)> {
        // [start, start+len) [range.dest,range.dest+range.len)
        // (StartA <= EndB) and (EndA >= StartB)
        if self.start < range.dest + range.len && self.start + self.len > range.dest {
            let start = if self.start < range.dest {
                range.dest + range.source - self.start
            } else {
                range.source + self.start - range.dest
            };
            let len = std::cmp::min(self.start + self.len, range.dest + range.len)
                - std::cmp::min(self.start, range.dest);
            Some((start, len))
        } else {
            None
        }
    }

    fn expand(&self, other: &SeedRange) -> SeedRange {
        if let Some(s) = self.overlap(&other) {
            let len = std::cmp::max(other.len, self.len);
            return SeedRange { start: s, len };
        }
        return SeedRange {
            start: self.start,
            len: self.len,
        };
    }
}

fn backward_range(start: &SeedRange, maps: &Vec<Vec<Range>>) -> SeedRange {
    let mut idx = maps.len() - 1;
    let mut seed = *start;
    loop {
        idx -= 1;
        for r in maps[idx].iter() {
            if let Some((start, len)) = seed.get_overlapping(r) {
                seed = seed.expand(&SeedRange { start, len });
            }
        }
        if idx == 0 {
            break;
        }
    }
    seed
}
