use std::str::FromStr;

pub fn part01(input: &str) -> i64 {
    let res = input
        .lines()
        .filter_map(|l| {
            return l.parse::<Card>().ok();
        })
        .map(|card| {
            return card;
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
    return 0;
}
#[derive(Debug)]
struct Card {
    nums: Vec<u32>,
    wins: Vec<u32>,
}

struct parseErr;
impl FromStr for Card {
    type Err = parseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let num_win
        let (_, num_win) = s.split_once(": ").ok_or(parseErr)?;
        let (left, right) = num_win.split_once(" | ").ok_or(parseErr)?;

        return Ok(Self {
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
