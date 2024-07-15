use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

#[derive(Clone, PartialEq, PartialOrd)]
struct Point {
    name: String,
    to: Vec<String>,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("name", &self.name)
            .field("to", &self.to)
            .finish()
    }
}

pub fn part01(input: &str) -> Option<i64> {
    let point_map = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(": ").unwrap();
            let to = r.split(" ").map(|s| s.to_string()).collect();
            let p = Point {
                name: l.to_string(),
                to,
            };
            return (l.to_string(), p);
        })
        .collect::<HashMap<String, Point>>();
    let mut new_map = HashMap::new();
    for p in point_map.values() {
        match new_map.entry(&p.name) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                let new_p: &mut Point = entry.get_mut();

                for to in &p.to {
                    if !new_p.to.contains(to) {
                        new_p.to.push(to.clone());
                    };
                }
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                let mut end_to = Vec::new();
                for to in &p.to {
                    if !end_to.contains(to) {
                        end_to.push(to.clone());
                    };
                }
                entry.insert(Point {
                    name: p.name.clone(),
                    to: end_to,
                });
            }
        }
        for to in &p.to {
            match new_map.entry(to) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    let end: &mut Point = entry.get_mut();
                    if !end.to.contains(&p.name) {
                        end.to.push(p.name.clone());
                    }
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    let mut end_to = Vec::new();
                    end_to.push(p.name.clone());
                    entry.insert(Point {
                        name: to.clone(),
                        to: end_to,
                    });
                }
            }
        }
    }
    let points: Vec<Point> = new_map.values().cloned().collect();
    // println!("{:?}", &points);

    let p = &points[0];
    let mut l = HashMap::new();
    l.insert(p.name.as_str(), p);
    let res = cut(&points[1..], &mut l, &mut HashMap::new());
    return Some(res as i64);
}

fn cut<'a>(
    points: &'a [Point],
    l: &mut HashMap<&'a str, &'a Point>,
    r: &mut HashMap<&'a str, &'a Point>,
) -> usize {
    if points.is_empty() {
        let mut connects = HashSet::new();
        l.iter()
            .flat_map(|(k, v)| {
                let t: Vec<(String, String)> =
                    v.to.iter()
                        .flat_map(|r_name| {
                            if r.contains_key(r_name.as_str()) {
                                return Some((k.to_string(), r_name.clone()));
                            }
                            None
                        })
                        .collect();
                t
            })
            .for_each(|t| {
                connects.insert(t);
            });
        if connects.len() == 3 {
            // println!("{:?}\n{:?}", &l.values(), &r.values());
            println!("{:?}", &connects);
            return l.len() * r.len();
        }
        return 0;
    }
    let next_p = &points[0];
    l.insert(&next_p.name, next_p);
    let add_l = cut(&points[1..], l, r);
    if add_l != 0 {
        return add_l;
    }
    l.remove(next_p.name.as_str());
    r.insert(&next_p.name, next_p);
    let add_r = cut(&points[1..], l, r);
    if add_r != 0 {
        return add_r;
    }
    r.remove(next_p.name.as_str());
    0
}

pub fn part02(input: &str) -> Option<i64> {
    return Some(0);
}
