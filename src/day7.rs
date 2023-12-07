const DATA: &str = include_str!("day7.txt");
use std::{cmp::Ordering, collections::HashMap};

pub fn part1() -> Option<()> {
    let mut cards = Vec::new();
    for line in DATA.lines() {
        let (a, b) = line.split_once(" ")?;
        cards.push((a, b.parse::<usize>().ok()?));
    }
    cards.sort_by(|a, b| compare(a.0, b.0));
    let mut result = 0;
    for (i, (_, b)) in cards.into_iter().enumerate() {
        result += (i + 1) * b;
    }
    println!("result = {result}");
    Some(())
}

pub fn part2() -> Option<()> {
    let mut cards = Vec::new();
    for line in DATA.lines() {
        let (a, b) = line.split_once(" ")?;
        cards.push((a, b.parse::<usize>().ok()?));
    }
    cards.sort_by(|a, b| joker_compare(a.0, b.0));
    let mut result = 0;
    for (i, (_, b)) in cards.into_iter().enumerate() {
        result += (i + 1) * b;
    }
    println!("result = {result}");
    Some(())
}

fn joker_compare(a: &str, b: &str) -> std::cmp::Ordering {
    let ak = joker_hand_kind(a);
    let bk = joker_hand_kind(b);

    let ck = ak.cmp(&bk);
    if ck == std::cmp::Ordering::Equal {
        for (aa, bb) in a.chars().zip(b.chars()) {
            let cc = joker_compare_card(aa, bb);
            if cc != Ordering::Equal {
                return cc;
            }
        }
    }
    return ck;
}

fn compare(a: &str, b: &str) -> std::cmp::Ordering {
    let ak = hand_kind(a);
    let bk = hand_kind(b);

    let ck = ak.cmp(&bk);
    if ck == std::cmp::Ordering::Equal {
        for (aa, bb) in a.chars().zip(b.chars()) {
            let cc = compare_card(aa, bb);
            if cc != Ordering::Equal {
                return cc;
            }
        }
    }
    return ck;
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum Kind {
    High,
    Pair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

fn hand_kind(a: &str) -> Kind {
    let mut cards = HashMap::new();
    for c in a.chars() {
        let e = cards.entry(c).or_insert(0);
        *e += 1;
    }
    match cards.len() {
        1 => Kind::Five,
        5 => Kind::High,
        2 => cards
            .into_values()
            .find(|x| *x == 4)
            .map_or(Kind::Full, |_| Kind::Four),
        3 => cards
            .into_values()
            .find(|x| *x == 3)
            .map_or(Kind::TwoPair, |_| Kind::Three),
        _ => Kind::Pair,
    }
}

fn joker_hand_kind(a: &str) -> Kind {
    if !a.contains('J') {
        return hand_kind(a);
    }

    let mut cards = HashMap::new();
    for c in a.chars() {
        let e = cards.entry(c).or_insert(0);
        *e += 1;
    }
    let joker_count = cards.remove(&'J').unwrap();

    match (cards.len(), joker_count) {
        (_, 5) => Kind::Five,
        (4, 1) => Kind::Pair,
        (3, 1) => Kind::Three,
        (3, 2) => Kind::Three,
        (2, 1) => cards
            .into_values()
            .find(|x| *x == 2)
            .map_or(Kind::Four, |_| Kind::Full),
        (2, 2) => Kind::Four,
        (2, 3) => Kind::Four,
        (1, _) => Kind::Five,
        _ => panic!("bad stuff: {:?}", cards),
    }
}

fn compare_card(a: char, b: char) -> Ordering {
    if a == b {
        return Ordering::Equal;
    }
    match (a, b) {
        ('A', _) => Ordering::Greater,
        (_, 'A') => Ordering::Less,
        ('K', _) => Ordering::Greater,
        (_, 'K') => Ordering::Less,
        ('Q', _) => Ordering::Greater,
        (_, 'Q') => Ordering::Less,
        ('J', _) => Ordering::Greater,
        (_, 'J') => Ordering::Less,
        ('T', _) => Ordering::Greater,
        (_, 'T') => Ordering::Less,
        _ => a.cmp(&b),
    }
}

fn joker_compare_card(a: char, b: char) -> Ordering {
    if a == b {
        return Ordering::Equal;
    }
    match (a, b) {
        ('J', _) => Ordering::Less,
        (_, 'J') => Ordering::Greater,
        ('A', _) => Ordering::Greater,
        (_, 'A') => Ordering::Less,
        ('K', _) => Ordering::Greater,
        (_, 'K') => Ordering::Less,
        ('Q', _) => Ordering::Greater,
        (_, 'Q') => Ordering::Less,
        ('T', _) => Ordering::Greater,
        (_, 'T') => Ordering::Less,
        _ => a.cmp(&b),
    }
}
