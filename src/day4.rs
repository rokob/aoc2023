const DATA: &str = include_str!("day4.txt");
use std::collections::HashSet;

pub fn part1() -> Option<()> {
    let mut result: u32 = 0;
    for line in DATA.lines() {
        let (win, card) = line.split_once(" | ")?;
        let win_nums = win
            .split_once(": ")?
            .1
            .split_ascii_whitespace()
            .collect::<HashSet<_>>();
        let card_nums = card.split_ascii_whitespace().collect::<Vec<_>>();
        let mut winning_count = 0;
        for num in card_nums {
            if win_nums.contains(num) {
                winning_count += 1;
            }
        }
        if winning_count > 0 {
            result += 2_u32.pow(winning_count - 1);
        }
    }
    println!("result = {result}");
    Some(())
}

pub fn part2() -> Option<()> {
    let mut cards = Vec::new();
    let mut card_counts = Vec::new();
    for line in DATA.lines() {
        let (win, card) = line.split_once(" | ")?;
        let (_, win_data) = win.split_once(": ")?;
        let win_nums = win_data.split_ascii_whitespace().collect::<HashSet<_>>();
        let card_nums = card.split_ascii_whitespace().collect::<Vec<_>>();
        let mut winning_count = 0;
        for num in card_nums {
            if win_nums.contains(num) {
                winning_count += 1;
            }
        }
        if winning_count > 0 {
            cards.push(winning_count);
        } else {
            cards.push(0);
        }
        card_counts.push(1);
    }
    for i in 0..cards.len() {
        let mult = cards[i] as usize;
        let prev_mult = card_counts[i];
        for j in 1..=mult {
            if i + j < cards.len() {
                card_counts[i + j] += prev_mult;
            }
        }
    }
    let result: u32 = card_counts.iter().sum();
    println!("result = {result}");
    Some(())
}
