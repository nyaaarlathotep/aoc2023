use std::fmt::Debug;

use itertools::Itertools;

#[derive(Clone, PartialEq, PartialOrd, Copy)]
struct HailStone {
    x: f64,
    y: f64,
    z: f64,
    v_x: f64,
    v_y: f64,
    v_z: f64,
}
impl Debug for HailStone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HailStone")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

pub fn part01(input: &str) -> Option<i64> {
    let start= 200000000000000_f64;
    let end= 400000000000000_f64;
    // let start = 7_f64;
    // let end = 27_f64;
    let total = collide(input, start, end);
    return Some(total as i64);
}

fn collide(input: &str, start: f64, end: f64) -> usize {
    let stones = parse_stones(input);

    let total = stones
        .iter()
        .enumerate()
        .map(|(i, hail_stone)| {
            stones[i..]
                .iter()
                .map(|another_stone| {
                    if cross(hail_stone, another_stone) < 0.0000000_f64 {
                        return 0;
                    }
                    let (v_x, v_y) = (
                        hail_stone.x - another_stone.x,
                        hail_stone.y - another_stone.y,
                    );
                    let t = get_offset(another_stone, v_y, v_x, hail_stone);
                    let (x, y) = (
                        hail_stone.x + t * hail_stone.v_x,
                        hail_stone.y + t * hail_stone.v_y,
                    );
                    if !(((x > hail_stone.x && hail_stone.v_x > 0.0000000_f64)
                        || (x < hail_stone.x && hail_stone.v_x < 0.0000000_f64))
                        && ((x > another_stone.x && another_stone.v_x > 0.0000000_f64)
                            || (x < another_stone.x && another_stone.v_x < 0.0000000_f64)))
                    {
                        return 0;
                    }
                    if x > start && x < end && y > start && y < end && t > 0.0000000_f64 {
                        // println!("{:?} -> {:?}:{:?}", hail_stone, another_stone, (x, y));
                        return 1;
                    }
                    return 0;
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    total
}

fn get_offset(another_stone: &HailStone, v_y: f64, v_x: f64, hail_stone: &HailStone) -> f64 {
    let t = (another_stone.v_x * v_y - v_x * another_stone.v_y) / cross(hail_stone, another_stone);
    t
}

fn cross(hail_stone: &HailStone, another_stone: &HailStone) -> f64 {
    hail_stone.v_x * another_stone.v_y - hail_stone.v_y * another_stone.v_x
}

fn parse_stones(input: &str) -> Vec<HailStone> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(" @ ").unwrap();
            let (x, y, z) = l
                .split(", ")
                .map(|num| {
                    return num.parse::<f64>().unwrap();
                })
                .collect_tuple()
                .unwrap();
            let (v_x, v_y, v_z) = r
                .split(", ")
                .map(|num| {
                    return num.trim().parse::<f64>().unwrap();
                })
                .collect_tuple()
                .unwrap();
            return HailStone {
                x,
                y,
                z,
                v_x,
                v_y,
                v_z,
            };
        })
        .collect()
}

pub fn part02(input: &str) -> Option<i64> {
    return Some(0);
}
