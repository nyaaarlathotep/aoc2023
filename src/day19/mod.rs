use std::collections::HashMap;

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
                        let cmp = if rule.contains(">") { Cmp::Gt } else { Cmp::Lt };
                        let (l, r) = match cmp {
                            Cmp::Gt => a.split_once(">").expect(">"),
                            Cmp::Lt => a.split_once("<").expect("<"),
                        };
                        let r = Rule {
                            direct: false,
                            name: l.to_string(),
                            cmp,
                            value: r.parse::<usize>().expect("usize"),
                            res: b.to_string(),
                        };
                        return r;
                    }
                    let r = Rule {
                        direct: true,
                        name: rule.to_string(),
                        cmp: Cmp::Gt,
                        value: 1,
                        res: "".to_string(),
                    };
                    return r;
                })
                .collect::<Vec<Rule>>();
            rule_map.insert(cap["name"].to_string(), rules);
        });
        let regex2 = Regex::new(r"(?m)^\{(?<part>.*)\}").unwrap();
        let mut total = 0;
        regex2.captures_iter(parts).for_each(|part| {
            let a: HashMap<&str, usize> = part["part"]
                .split(",")
                .map(|p| {
                    let a = p.split_once("=").unwrap();
                    return (a.0, a.1.parse::<usize>().expect("????"));
                })
                .collect::<HashMap<&str, usize>>();
            // println!("{:?}", &a);

            let next_name = "in";
            let res = get_res(&rule_map, &next_name, &a);
            if res == "A" {
                let this_res = a
                    .iter()
                    .map(|(_k, v)| {
                        return *v;
                    })
                    .sum::<usize>();
                total += this_res;
            }
        });
        println!("{}",total);
        return Some(total as i64);
    });

    return Some(0);
}

fn get_res<'a>(
    rule_map: &'a HashMap<String, Vec<Rule>>,
    next_name: &'a str,
    part: &HashMap<&str, usize>,
) -> &'a str {
    if next_name == "A" || next_name == "R" {
        return next_name;
    }
    let rules = rule_map.get(next_name).expect("?????");
    for r in rules {
        if r.direct {
            // println!("->{}", &r.name);
            return get_res(&rule_map, &r.name, part);
        } else {
            match r.cmp {
                Cmp::Gt => {
                    if part.get(r.name.as_str()).unwrap() > &r.value {
                        // println!("->{}", &r.res);
                        return get_res(&rule_map, &r.res, part);
                    }
                }
                Cmp::Lt => {
                    if part.get(r.name.as_str()).unwrap() < &r.value {
                        // println!("->{}", &r.res);
                        return get_res(&rule_map, &r.res, part);
                    }
                }
            }
        }
    }
    panic!("????");
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
    Gt,
    Lt,
}
