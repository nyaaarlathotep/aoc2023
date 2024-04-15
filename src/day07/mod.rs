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
            for i in [0..5] {
                if a.hand.get(i.clone()) > b.hand.get(i.clone()) {
                    return Ordering::Greater;
                } else if a.hand.get(i.clone()) < b.hand.get(i.clone()) {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        })
        .rev()
        .collect();
    let res: u64 = cards
        .iter()
        .enumerate()
        .map(|(i, card)| {
            let var_name = (i+1) as u64 *card.value as u64;
            return var_name;
        })
        .sum::<u64>();
    println!("{:?}", cards);
    return res as i64;
}

pub fn part02(input: &str) -> i64 {
    return 0;
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    pos: u32,
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
                hand: dest,
                value: b.parse::<u32>().unwrap(),
            }
        });
    }
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
        return 1;
    }
    if len == 2 {
        if card_map
            .values()
            .into_iter()
            .any(|count| return *count == 4)
        {
            return 2;
        }
        if card_map
            .values()
            .into_iter()
            .any(|count| return *count == 3)
        {
            return 3;
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
        return 5;
    }
    if len == 4 {
        return 6;
    }
    return 7;
}
