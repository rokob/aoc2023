const DATA: &str = include_str!("day2.txt");

#[derive(Default)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn add(&mut self, color: &str, count: u32) {
        match color {
            "red" => self.red += count,
            "green" => self.green += count,
            "blue" => self.blue += count,
            _ => {}
        }
    }
}

struct Game {
    id: usize,
    rounds: Vec<Draw>,
}

fn parse_games() -> Option<Vec<Game>> {
    let mut games = Vec::new();
    for line in DATA.lines() {
        let (name, data) = line.split_once(": ")?;
        let id = name.split_once(" ")?.1.parse::<usize>().ok()?;
        let mut rounds = Vec::new();
        for drawdata in data.split("; ") {
            let draw = drawdata
                .split(", ")
                .map(|s| {
                    let (count, color) = s.split_once(" ").unwrap();
                    (count.parse::<u32>().unwrap(), color)
                })
                .fold(Draw::default(), |mut acc, (n, c)| {
                    acc.add(c, n);
                    acc
                });
            rounds.push(draw);
        }
        games.push(Game { id, rounds });
    }
    Some(games)
}

pub fn part1() -> Option<()> {
    let games = parse_games()?;
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    let mut result = 0;
    for game in games.iter() {
        let mut valid = true;
        for draw in game.rounds.iter() {
            if draw.red > 12 || draw.green > 13 || draw.blue > 14 {
                valid = false;
                break;
            }
        }
        if valid {
            result += game.id
        }
    }
    println!("result: {result}");
    Some(())
}

pub fn part2() -> Option<()> {
    let games = parse_games()?;
    let mut result = 0;
    for game in games.iter() {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for draw in game.rounds.iter() {
            if draw.red > min_red {
                min_red = draw.red;
            }
            if draw.blue > min_blue {
                min_blue = draw.blue;
            }
            if draw.green > min_green {
                min_green = draw.green;
            }
        }
        let power = min_red * min_blue * min_green;
        result += power;
    }
    println!("result: {result}");
    Some(())
}
