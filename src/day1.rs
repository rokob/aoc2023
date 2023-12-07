const DATA: &str = include_str!("day1part1.txt");

pub fn part1() {
    let mut result = 0;
    for line in DATA.lines() {
        let nums: Vec<_> = line.chars().flat_map(|x| x.to_digit(10)).collect();
        let a = nums[0];
        let b = nums[nums.len() - 1];
        result += 10 * a + b;
    }
    println!("result: {result}");
}

pub fn part2() {
    let mut result = 0;
    for line in DATA.lines() {
        let cs = line.chars().collect();
        let a = find_first(&cs);
        let b = find_last(&cs);
        result += 10 * a + b;
    }
    println!("result: {result}");
}

fn find_first(v: &Vec<char>) -> i32 {
    for i in 0..v.len() {
        if v[i].is_digit(10) {
            return v[i].to_digit(10).unwrap() as i32;
        }
        if let Some((c, _)) = word_number_at_index(v, i) {
            return c.to_digit(10).unwrap() as i32;
        }
    }
    panic!("no numbers");
}

fn find_last(v: &Vec<char>) -> i32 {
    let mut i = v.len() - 1;
    loop {
        if v[i].is_digit(10) {
            return v[i].to_digit(10).unwrap() as i32;
        }
        if let Some((c, _)) = word_number_at_index(v, i) {
            return c.to_digit(10).unwrap() as i32;
        }
        i -= 1;
    }
}

fn word_number_at_index(v: &Vec<char>, i: usize) -> Option<(char, usize)> {
    let nums = [
        ("one", '1', 3),
        ("two", '2', 3),
        ("three", '3', 5),
        ("four", '4', 4),
        ("five", '5', 4),
        ("six", '6', 3),
        ("seven", '7', 5),
        ("eight", '8', 5),
        ("nine", '9', 4),
    ];
    for (needle, out, len) in nums {
        if is_string_at_index(v, needle, i) {
            return Some((out, len));
        }
    }
    None
}

fn is_string_at_index(v: &Vec<char>, needle: &str, i: usize) -> bool {
    let cs = needle.chars().collect::<Vec<_>>();
    for k in i..i + needle.len() {
        if k >= v.len() {
            return false;
        }
        if v[k] != cs[k - i] {
            return false;
        }
    }
    true
}
