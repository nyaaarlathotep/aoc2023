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
        f.debug_struct("Point").field("name", &self.name).finish()
    }
}

pub fn part01(input: &str) -> Option<i64> {
    let points: Vec<Point> = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(": ").unwrap();
            let to = r.split(" ").map(|s| s.to_string()).collect();
            let p = Point {
                name: l.to_string(),
                to,
            };
            return p;
        })
        .collect::<Vec<Point>>();
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
                let t:Vec<(String,String)> =
                    v.to.iter()
                        .flat_map(|name| {
                            if let Some(r_name) = r.get(name.as_str()) {
                                return Some((name.clone(), r_name.name.clone()));
                            }
                            None
                        })
                        .collect();
                t
            })
            .for_each(|t| {
                connects.insert(t);
            });
        r.iter()
            .flat_map(|(k, v)| {
                let t: Vec<(String, String)> =
                    v.to.iter()
                        .filter_map(|name| {
                            if let Some(l_name) = l.get(name.as_str()) {
                                return Some((l_name.name.clone(), name.clone()));
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
            println!("{:?}\n{:?}", &l.values(), &r.values());
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
        return add_l;
    }
    r.remove(next_p.name.as_str());
    0
}

pub fn part02(input: &str) -> Option<i64> {
    return Some(0);
}
