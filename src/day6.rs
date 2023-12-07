const DATA: &str = include_str!("day6.txt");

pub fn part1() -> Option<()> {
    let mut data = DATA.lines();
    let times = data
        .next()?
        .split_once(": ")?
        .1
        .split_whitespace()
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let dists = data
        .next()?
        .split_once(": ")?
        .1
        .split_whitespace()
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let races = times.into_iter().zip(dists.into_iter()).collect::<Vec<_>>();

    let mut result = 1;
    for (t, d) in races.into_iter() {
        let mut count = 0;
        for n in 0..=t {
            if n * (t - n) > d {
                count += 1;
            }
        }
        result *= count;
    }
    println!("result = {result}");

    Some(())
}

pub fn part2() -> Option<()> {
    let mut data = DATA.lines();
    let times = data
        .next()?
        .split_once(": ")?
        .1
        .split_whitespace()
        .map(|x| x.trim())
        .collect::<Vec<_>>();

    let time = times.join("").parse::<usize>().ok()?;

    let dists = data
        .next()?
        .split_once(": ")?
        .1
        .split_whitespace()
        .map(|x| x.trim())
        .collect::<Vec<_>>();

    let dist = dists.join("").parse::<usize>().ok()?;

    let result = compute(time, dist);
    println!("result {result}");

    Some(())
}

fn compute(time: usize, dist: usize) -> usize {
    let det = time * time - 4 * dist;
    let fac = (det as f64).sqrt() / 2_f64;
    let fixed = time as f64 / 2_f64;
    let low_n = (fixed - fac).ceil() as usize;
    let high_n = (fixed + fac).floor() as usize;

    high_n - low_n + 1
}
