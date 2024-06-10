use std::collections::HashMap;

use regex::Regex;

pub fn part01(input: &str) -> Option<i64> {
    input.split_once("\n\n").map(|(rules, parts)| {
        let rule_map: HashMap<&str, Vec<fn(Part) -> &str>> = HashMap::new();
        let regex = Regex::new(r"(?m)^(?<name>[\w+]+)\{(?<cons>.*)\}").unwrap();
        regex.captures_iter(rules).for_each(|cap| {
            println!("{:?}", &cap["name"]);
            println!("{:?}", &cap["cons"]);
            // let rules: Vec<fn(Part) -> &str> = &cap["cons"]
            let rules: &Vec<Box<dyn Fn(Part) -> &str>> = &cap["cons"]
                .split(",")
                .map(|rule| {
                    // if !rule.contains(":") {
                    //     return Box::new(|part| {
                    //         return rule;
                    //     });
                    // }
                 
                })
                .collect();
        })
    });
    return Some(0);
}

pub fn part02(input: &str) -> Option<i64> {
    return Some(0);
}

struct Rules{

}

struct Part<'a> {
    x: &'a str,
    m: &'a str,
    a: &'a str,
    s: &'a str,
}
