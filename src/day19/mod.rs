use std::{collections::HashMap, hash::Hash};

use regex::Regex;

pub fn part01(input: &str) -> Option<i64> {
    input.split_once("\n\n").map(|(rules, parts)| {
        let mut rule_map: HashMap<String, Vec<Rule>> = HashMap::new();
        let regex = Regex::new(r"(?m)^(?<name>[\w+]+)\{(?<cons>.*)\}").unwrap();
        regex.captures_iter(rules).for_each(|cap| {
            // println!("{:?}", &cap["name"]);
            // println!("{:?}", &cap["cons"]);
            let rules = cap["cons"]
                .split(",")
                .map(|rule| {
                    if rule.contains(":") {
                        let (a, b) = rule.split_once(":").expect("!");
                        let cmp = if rule.contains(">") { Cmp::gt } else { Cmp::lt };
                        let (l, r) = match cmp {
                            Cmp::gt => a.split_once(">").expect(">"),
                            Cmp::lt => a.split_once("<").expect("<"),
                        };
                        let r = Rule {
                            direct: true,
                            name: l.to_string(),
                            cmp,
                            value: r.parse::<usize>().expect("usize"),
                            res: b.to_string(),
                        };
                        return r;
                    }
                    let r = Rule {
                        direct: false,
                        name: "".to_string(),
                        cmp: Cmp::gt,
                        value: 1,
                        res: "".to_string(),
                    };
                    return r;
                })
                .collect::<Vec<Rule>>();
            rule_map.insert(cap["name"].to_string(), rules);
        });
        let regex2 = Regex::new(r"(?m)^\{(?<part>.*)\}").unwrap();
        let mut res = 0;
        regex2.captures_iter(parts).for_each(|part| {
            let a: HashMap<&str, usize> = part["part"]
                .split(",")
                .map(|p| {
                    let a = p.split_once("=").unwrap();
                    return (a.0, a.1.parse::<usize>().expect("????"));
                })
                .collect::<HashMap<&str, usize>>();
            println!("{:?}", &a);
        });
    });

    return Some(0);
}

pub fn part02(input: &str) -> Option<i64> {
    return Some(0);
}

struct Rule {
    direct: bool,
    name: String,
    cmp: Cmp,
    value: usize,
    res: String,
}

enum Cmp {
    gt,
    lt,
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
