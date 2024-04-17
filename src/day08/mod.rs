use std::{collections::HashMap, intrinsics::mir::Len, str::FromStr};

pub fn part01(input: &str) -> i64 {
    // input.split_once("\n\n").map(|(instract,map)|{

    // })
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
        let mut ins;
        s.split_once("\n\n").map(|(up, down)| {
            ins = up;
            down.lines().map(|line| {
                line.split_once(" = ").map(|(start, end)| {
                    end.trim_matches(|char| char == '(' || char == ')')
                        .split_once(", ")
                        .map(|(l, r)| {
                            map.insert(start.to_string(), (l.to_string(), r.to_string()));
                        });
                })
            });
        });
        Ok(Self {
            instrcts: ins.to_string(),
            map: map,
        })
    }
}

pub fn part02(input: &str) -> i64 {
    return 0;
}
