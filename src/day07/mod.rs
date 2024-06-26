use core::panic;
use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use itertools::Itertools;

pub fn part01(input: &str) -> i64 {
    let mut num_map: HashMap<u8, u32> = HashMap::new();
    num_map.insert('2' as u8, 2);
    num_map.insert('3' as u8, 3);
    num_map.insert('4' as u8, 4);
    num_map.insert('5' as u8, 5);
    num_map.insert('6' as u8, 6);
    num_map.insert('7' as u8, 7);
    num_map.insert('8' as u8, 8);
    num_map.insert('9' as u8, 9);
    num_map.insert('T' as u8, 10);
    num_map.insert('J' as u8, 11);
    num_map.insert('Q' as u8, 12);
    num_map.insert('K' as u8, 13);
    num_map.insert('A' as u8, 14);
    let cards: Vec<Card> = input
        .lines()
        .filter_map(|line| {
            return line.parse::<Card>().ok();
        })
        .sorted_by(|a, b| {
            if a.pos > b.pos {
                return Ordering::Greater;
            } else if a.pos < b.pos {
                return Ordering::Less;
            }
            for i in 0..5 {
                let a_value = num_map.get(a.hand.get(i).unwrap());
                let b_value = num_map.get(b.hand.get(i).unwrap());
                if a_value > b_value {
                    return Ordering::Greater;
                } else if a_value < b_value {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        })
        .collect();
    let res: u64 = cards
        .iter()
        .enumerate()
        .map(|(i, card)| {
            let vvvv = (i + 1) as u64 * card.value as u64;
            return vvvv;
        })
        .sum::<u64>();
    return res as i64;
}

pub fn part02(input: &str) -> i64 {
    let mut num_map: HashMap<u8, u32> = HashMap::new();
    num_map.insert('2' as u8, 2);
    num_map.insert('3' as u8, 3);
    num_map.insert('4' as u8, 4);
    num_map.insert('5' as u8, 5);
    num_map.insert('6' as u8, 6);
    num_map.insert('7' as u8, 7);
    num_map.insert('8' as u8, 8);
    num_map.insert('9' as u8, 9);
    num_map.insert('T' as u8, 10);
    num_map.insert('J' as u8, 1);
    num_map.insert('Q' as u8, 12);
    num_map.insert('K' as u8, 13);
    num_map.insert('A' as u8, 14);
    let cards: Vec<JokerCard> = input
        .lines()
        .filter_map(|line| {
            return line.parse::<JokerCard>().ok();
        })
        .sorted_by(|a, b| {
            if a.pos > b.pos {
                return Ordering::Greater;
            } else if a.pos < b.pos {
                return Ordering::Less;
            }
            for i in 0..5 {
                let a_value = num_map.get(a.hand.get(i).unwrap());
                let b_value = num_map.get(b.hand.get(i).unwrap());
                if a_value > b_value {
                    return Ordering::Greater;
                } else if a_value < b_value {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        })
        .collect();
    let res: u64 = cards
        .iter()
        .enumerate()
        .map(|(i, card)| {
            let vvvv = (i + 1) as u64 * card.value as u64;
            return vvvv;
        })
        .sum::<u64>();
    return res as i64;
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    pos: u32,
    origin: String,
    hand: [u8; 5],
    value: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct JokerCard {
    pos: u32,
    origin: String,
    hand: [u8; 5],
    value: u32,
}

struct ParseErr;
impl FromStr for Card {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(" ").unwrap();
        let mut dest: [u8; 5] = [0, 0, 0, 0, 0];
        dest.copy_from_slice(a.as_bytes());
        return Ok({
            Self {
                pos: get_pos(a),
                origin: a.to_string(),
                hand: dest,
                value: b.parse::<u32>().unwrap(),
            }
        });
    }
}
impl FromStr for JokerCard {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(" ").unwrap();
        let mut dest: [u8; 5] = [0, 0, 0, 0, 0];
        dest.copy_from_slice(a.as_bytes());
        return Ok({
            Self {
                pos: get_pos_part2(a),
                origin: a.to_string(),
                hand: dest,
                value: b.parse::<u32>().unwrap(),
            }
        });
    }
}
fn get_pos_part2(s: &str) -> u32 {
    let mut card_map: HashMap<char, u32> = HashMap::new();
    for r in s.chars() {
        if let Some(count) = card_map.get(&r) {
            card_map.insert(r, count + 1);
        } else {
            card_map.insert(r, 1);
        }
    }
    println!("before: {:?}",card_map);
    card_map.remove(&'J').map(|joker_count| {
        if joker_count==5 {
            card_map.insert('J', joker_count);
        }
        let mut max_count = &0;
        let mut max_key = &'m';
        for k in card_map.keys() {
            if card_map.get(k).unwrap() > max_count {
                max_count = card_map.get(k).unwrap();
                max_key = k;
            }
        }
        if max_key==&'m' {
            panic!("???");
        }
        card_map.insert(*max_key, card_map.get(max_key).unwrap() + joker_count);
    });
    let len = card_map.keys().len();
    if len == 1 {
        return 7;
    }
    if len == 2 {
        if card_map
            .values()
            .into_iter()
            .any(|count| return *count == 4)
        {
            return 6;
        }
        if card_map
            .values()
            .into_iter()
            .any(|count| return *count == 3)
        {
            return 5;
        }
        panic!("?{:?}", card_map)
    }
    if len == 3 {
        if card_map
            .values()
            .into_iter()
            .any(|count| return *count == 3)
        {
            return 4;
        }
        return 3;
    }
    if len == 4 {
        return 2;
    }
    return 1;
}

fn get_pos(s: &str) -> u32 {
    let mut card_map: HashMap<char, u32> = HashMap::new();
    for r in s.chars() {
        if let Some(count) = card_map.get(&r) {
            card_map.insert(r, count + 1);
        } else {
            card_map.insert(r, 1);
        }
    }
    let len = card_map.keys().len();
    if len == 1 {
        return 7;
    }
    if len == 2 {
        if card_map
            .values()
            .into_iter()
            .any(|count| return *count == 4)
        {
            return 6;
        }
        if card_map
            .values()
            .into_iter()
            .any(|count| return *count == 3)
        {
            return 5;
        }
        panic!("?{:?}", card_map)
    }
    if len == 3 {
        if card_map
            .values()
            .into_iter()
            .any(|count| return *count == 3)
        {
            return 4;
        }
        return 3;
    }
    if len == 4 {
        return 2;
    }
    return 1;
}
