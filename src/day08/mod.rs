use std::{collections::HashMap, str::FromStr};

pub fn part01(input: &str) -> i64 {
    let network = input.parse::<Network>().ok().unwrap();
    let mut steps = 0;
    let mut pos_now = "AAA".to_string();
    for c in network.instrcts.chars().cycle() {
        if pos_now == "ZZZ" {
            return steps as i64;
        }
        steps = steps + 1;
        let next_poses = network.map.get(&pos_now).unwrap();
        match c {
            'L' => {
                pos_now = next_poses.0.clone();
            }
            'R' => {
                pos_now = next_poses.1.clone();
            }
            _ => {
                panic!("{:?}", c);
            }
        }
    }
    unreachable!()
}

#[derive(Debug, PartialEq, Eq)]
struct Network {
    instrcts: String,
    map: HashMap<String, (String, String)>,
}

struct ParseErr;
impl FromStr for Network {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<String, (String, String)> = HashMap::new();
        let mut ins = "";
        s.split_once("\n\n").map(|(up, down)| {
            ins = up;
            down.lines().for_each(|line| {
                line.split_once(" = ").map(|(start, end)| {
                    end.trim_matches(|char| char == '(' || char == ')')
                        .split_once(", ")
                        .map(|(l, r)| {
                            map.insert(start.to_string(), (l.to_string(), r.to_string()));
                        });
                });
            });
        });
        Ok(Self {
            instrcts: ins.to_string(),
            map,
        })
    }
}

pub fn part02(input: &str) -> i64 {
    let network = input.parse::<Network>().ok().unwrap();
    let mut steps: Vec<i64> = Vec::new();
    for pos in network.map.keys() {
        if pos.ends_with("A") {
            steps.push(get_steps(&network, pos));
        }
    }
    return steps.into_iter().fold(1 as i64, |a, b| {
        return lcm(a, b);
    });
}

fn gcd(a: i64, b: i64) -> i64 {
    if a % b == 0 {
        return b;
    }
    return gcd(b, a % b);
}

fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gcd(a, b);
}

fn get_steps(network: &Network, start: &String) -> i64 {
    let mut steps = 0;
    let mut pos_now = start.clone();
    for c in network.instrcts.chars().cycle() {
        if pos_now.ends_with("Z") {
            return steps;
        }
        steps = steps + 1;
        let next_poses = network.map.get(&pos_now).unwrap();
        match c {
            'L' => {
                pos_now = next_poses.0.clone();
            }
            'R' => {
                pos_now = next_poses.1.clone();
            }
            _ => {
                panic!("{:?}", c);
            }
        }
    }
    unreachable!();
}
