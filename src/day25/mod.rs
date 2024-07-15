use core::panic;
use std::collections::HashMap;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
struct Point {
    name: String,
    to: Vec<String>,
}

pub fn part01(input: &str) -> Option<i64> {
    let mut points_map: HashMap<&str, Point> = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(": ").unwrap();
            let to = r.split(" ").map(|s| s.to_string()).collect();
            let p = Point {
                name: l.to_string(),
                to,
            };
            return (l, p);
        })
        .collect::<HashMap<&str, Point>>();
    let mut points: Vec<Point> = Vec::new();
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
        let connect_lines: usize = l
            .iter()
            .map(|(k, v)| {
                let t =
                    v.to.iter()
                        .map(|name| {
                            if r.contains_key(name.as_str()) {
                                return 1;
                            }
                            return 0;
                        })
                        .sum::<usize>();
                t
            })
            .sum();
        print!("{}", connect_lines);
        if connect_lines == 3 {
            println!("{:?}\n{:?}", &l, &r);
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
