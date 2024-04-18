use std::{collections::HashMap,  str::FromStr};

pub fn part01(input: &str) -> i64 {
    let network = input.parse::<Network>().ok().unwrap();
    println!("{:?}", network);
    let mut steps = 0;
    let mut pos_now = "AAA";
    // let mut fimilar_scece: HashMap<usize, Vec<String>> = HashMap::new();
    for c in network.instrcts.chars().cycle(){
     if pos_now== "ZZZ" {
            return steps as i64;
        }
        let next_pos=network.map.get(pos_now).unwrap();
        match c {
            'L' => {
                
            }
            'R' => {}
            _ => {
                panic!("{:?}", c);
            }
        }
        // pos_now=network.map.get(pos_now)
        // fimilar_scece.get(&i).map(|vec| {});
    }
    return 0;
}

#[derive(Debug, PartialEq, Eq)]
struct Network {
    instrcts: String,
    map: HashMap<String, (String, String)>,
}

struct parseErr;
impl FromStr for Network {
    type Err = parseErr;
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
    return 0;
}
