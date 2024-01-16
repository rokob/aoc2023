const DATA: &str = include_str!("day24.txt");
use z3::ast::{Ast, Int};
use z3::{Config, Context, Solver};

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
    let hail = parse(DATA);

    let result = solve_3d(hail);
    println!("result = {result}");
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

fn solve_3d(hail: Vec<Hail>) -> String {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let px = Int::new_const(&ctx, "px");
    let py = Int::new_const(&ctx, "py");
    let pz = Int::new_const(&ctx, "pz");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    for stone in hail {
        let pxn = Int::from_i64(&ctx, stone.pos.x as i64);
        let pyn = Int::from_i64(&ctx, stone.pos.y as i64);
        let pzn = Int::from_i64(&ctx, stone.pos.z as i64);
        let vxn = Int::from_i64(&ctx, stone.vel.x as i64);
        let vyn = Int::from_i64(&ctx, stone.vel.y as i64);
        let vzn = Int::from_i64(&ctx, stone.vel.z as i64);
        let tn = Int::fresh_const(&ctx, "t");

        solver.assert(&(&pxn + &vxn * &tn)._eq(&(&px + &vx * &tn)));
        solver.assert(&(&pyn + &vyn * &tn)._eq(&(&py + &vy * &tn)));
        solver.assert(&(&pzn + &vzn * &tn)._eq(&(&pz + &vz * &tn)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&px).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&py).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&pz).unwrap().as_i64().unwrap();

    (x + y + z).to_string()
}
