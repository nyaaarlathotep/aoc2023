use std::{collections::HashMap, str::FromStr};

pub fn part01(input: &str) -> i64 {
    let res = input
        .lines()
        .filter_map(|l| {
            return l.parse::<Card>().ok();
        })
        .map(|card| {
            let mut hit_num = 0;
            for num in card.nums.as_slice() {
                for win_num in card.wins.as_slice() {
                    if win_num == num {
                        hit_num = hit_num + 1;
                        break;
                    }
                }
            }
            return hit_num;
        })
        .map(|hit_num| {
            if hit_num == 0 {
                return 0;
            }
            return 2_i64.pow(hit_num - 1);
        })
        .sum::<i64>();
    return res;
}

pub fn part02(input: &str) -> i64 {
    let mut card_copy_map: HashMap<u32, i64> = HashMap::new();
    let res = input
        .lines()
        .filter_map(|l| {
            return l.parse::<Card>().ok();
        })
        .map(|card| {
            let mut hit_num = 0;
            for num in card.nums.as_slice() {
                for win_num in card.wins.as_slice() {
                    if win_num == num {
                        hit_num = hit_num + 1;
                        break;
                    }
                }
            }
            let this_count;
            match card_copy_map.get(&(card.index)) {
                Some(copys) => {
                    this_count = copys + 1;
                }
                None => {
                    this_count = 1;
                }
            };
            for i in 1..hit_num + 1 {
                match card_copy_map.get(&(card.index + i)) {
                    Some(copys) => {
                        card_copy_map.insert(card.index + i, copys + this_count);
                    }
                    None => {
                        card_copy_map.insert(card.index + i, this_count);
                    }
                };
            }
            return this_count;
        })
        .sum::<i64>();
    res
}

#[derive(Debug)]
struct Card {
    index: u32,
    nums: Vec<u32>,
    wins: Vec<u32>,
}

struct ParseErr;
impl FromStr for Card {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let num_win
        let (index, num_win) = s.split_once(": ").ok_or(ParseErr)?;
        let (left, right) = num_win.split_once(" | ").ok_or(ParseErr)?;
        let count = index.split(" ").last().ok_or(ParseErr)?;
        return Ok(Self {
            index: count.parse::<u32>().unwrap(),
            nums: left
                .split(" ")
                .filter(|&s| {
                    return !s.eq("");
                })
                .map(|s| {
                    return s.parse::<u32>().unwrap();
                })
                .collect(),
            wins: right
                .split(" ")
                .filter(|&s| {
                    return !s.eq("");
                })
                .map(|s| {
                    return s.parse::<u32>().unwrap();
                })
                .collect(),
        });
    }
}
