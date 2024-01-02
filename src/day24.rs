const DATA: &str = include_str!("day24.txt");

#[derive(Debug, Clone, PartialEq)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct Hail {
    pos: Vector,
    vel: Vector,
}

impl FromIterator<f64> for Vector {
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        let mut i = iter.into_iter();
        let (x, y, z) = (i.next().unwrap(), i.next().unwrap(), i.next().unwrap());
        Vector { x, y, z }
    }
}

pub fn part1() -> Option<()> {
    let hail = parse(DATA);
    let min = 200000000000000f64;
    let max = 400000000000000f64;

    let mut count = 0;
    for i in 0..hail.len() - 1 {
        for j in i + 1..hail.len() {
            match intersect2d(&hail[i], &hail[j]) {
                Some((x, y)) if x >= min && x <= max && y >= min && y <= max => count += 1,
                _ => {}
            }
        }
    }

    println!("result = {count}");

    Some(())
}

pub fn part2() -> Option<()> {
    Some(())
}

fn parse(input: &str) -> Vec<Hail> {
    input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once(" @ ").unwrap();
            let pos: Vector = pos.split(", ").map(|v| v.parse::<f64>().unwrap()).collect();
            let vel: Vector = vel.split(", ").map(|v| v.parse::<f64>().unwrap()).collect();
            Hail { pos, vel }
        })
        .collect()
}

fn intersect2d(a: &Hail, b: &Hail) -> Option<(f64, f64)> {
    // (a.pos.x + t*a.vel.x, a.pos.y + t*a.vel.y)
    // (b.pos.x + t*b.vel.x, b.pos.y + t*b.vel.y)
    //
    // t = (a.pos.x - b.pos.x) / (b.vel.x - a.vel.x)
    // y - y1 = m(x - x1)
    // y - a.pos.y = (a.vel.y / a.vel.x) * (x - a.pos.x)
    // a.pos.y + (a.vel.y / a.vel.x) * (x - a.pos.x) = b.pos.y + (b.vel.y / b.vel.x) * (x - b.pos.x)
    //
    // a.pos.y - b.pos.y + (mb*b.pos.x - ma*a.pos.x) = x(mb - ma)
    // x = (1/(mb - ma))*(a.pos.y - b.pos.y + (mb*b.pos.x - ma*a.pos.x)

    let ma = a.vel.y / a.vel.x;
    let mb = b.vel.y / b.vel.x;

    if mb == ma {
        return None;
    }

    let x = (a.pos.y - b.pos.y + mb * b.pos.x - ma * a.pos.x) / (mb - ma);
    let y = a.pos.y + ma * (x - a.pos.x);
    let ta = (x - a.pos.x) / a.vel.x;
    let tb = (x - b.pos.x) / b.vel.x;

    if ta < 0f64 || tb < 0f64 {
        return None;
    }

    Some((x, y))
}
