use std::{
    fmt::{Debug, Display},
    str::FromStr,
    usize,
};

use itertools::Itertools;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
struct Interval {
    low: usize,
    high: usize,
}
impl Interval {
    const fn intersects(self, rhs: &Self) -> bool {
        self.low > rhs.high || self.high < rhs.low
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd)]
struct Block {
    z: Interval,
    y: Interval,
    x: Interval,
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.z.low, self.z.high)?;
        Ok(())
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z.low.cmp(&other.z.low)
    }
}

impl Block {
    const fn overlaps_xy(&self, block: &Self) -> bool {
        !self.x.intersects(&block.x) && !self.y.intersects(&block.y)
    }

    const fn overlaps_xyz(&self, block: &Self) -> bool {
        self.overlaps_xy(block) && !self.z.intersects(&block.z)
    }

    const fn high_point(&self, block: &Self) -> Option<usize> {
        if self.overlaps_xy(block) {
            Some(self.z.high)
        } else {
            None
        }
    }

    fn move_down(&mut self, down: usize) {
        self.z.low -= down;
        self.z.high -= down;
    }
    fn get_blocks_above<'a>(&self, blocks: &'a [Self]) -> Vec<&'a Self> {
        let mut one_up = self.clone();
        one_up.z.low += 1;
        one_up.z.high += 1;
        blocks
            .iter()
            .filter(|x| *x != self && one_up.overlaps_xyz(x))
            .collect()
    }
    fn count_supports(&self, blocks: &[Self]) -> usize {
        let mut one_down = self.clone();
        one_down.z.low -= 1;
        one_down.z.high -= 1;
        blocks
            .iter()
            .filter(|x| *x != self)
            .filter(|x| one_down.overlaps_xyz(x))
            .count()
    }
}

struct ParseErr;
impl FromStr for Block {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, e) = s.split_once("~").unwrap();
        let start: (usize, usize, usize) = s
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let end: (usize, usize, usize) = e
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .unwrap();
        return Ok(Block {
            z: Interval {
                low: start.2.min(end.2),
                high: start.2.max(end.2),
            },
            y: Interval {
                low: start.1.min(end.1),
                high: start.1.max(end.1),
            },
            x: Interval {
                low: start.0.min(end.0),
                high: start.0.max(end.0),
            },
        });
    }
}

pub fn part01(input: &str) -> Option<i64> {
    let mut blocks: Vec<Block> = input
        .lines()
        .map(|s| {
            return s.parse::<Block>().ok().unwrap();
        })
        .collect();

    blocks.sort();
    adjust_blocks(&mut blocks);

    let free_blocks = get_free_blocks(&blocks);

    return Some(free_blocks.len() as i64);
}

fn get_free_blocks(blocks: &Vec<Block>) -> Vec<&Block> {
    blocks
        .into_iter()
        .filter(|b| {
            let above = b.get_blocks_above(&blocks);
            if above.is_empty() {
                return true;
            }
            above.iter().all(|y| y.count_supports(&blocks) > 1)
        })
        .collect::<Vec<&Block>>()
}

fn adjust_blocks(blocks: &mut Vec<Block>) {
    let down = blocks[0].z.low - 1;
    blocks[0].move_down(down);
    for i in 1..blocks.len() {
        let (set, unset) = blocks.split_at_mut(i);
        let move_b = &mut unset[0];
        let down = set
            .iter()
            .filter_map(|b| Some(b.high_point(&move_b)? + 1))
            .max()
            .unwrap_or(1);
        move_b.move_down(move_b.z.low - down);
    }
}

pub fn part02(input: &str) -> Option<i64> {
    return Some(0);
}
