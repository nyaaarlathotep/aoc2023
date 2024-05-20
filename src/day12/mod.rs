use std::{collections::HashMap, hash::Hash, iter};

pub fn part01(input: &str) -> i64 {
    return input
        .lines()
        .map(|l| {
            let (field, spring_group): (&str, Vec<usize>) = l
                .split_once(" ")
                .map(|(field, r)| {
                    (
                        field,
                        r.split(",")
                            .into_iter()
                            .map(|n| {
                                return n.parse::<usize>().expect("???");
                            })
                            .collect(),
                    )
                })
                .unwrap();
            // let field_pieces: Vec<&str> = field
            //     .split(".")
            //     .filter(|w| {
            //         return w.len() != 0;
            //     })
            //     .collect();
            let mut cache = HashMap::new();
            let total = possible_solutions(field, &spring_group, &mut cache);
            return total;
        })
        .sum::<usize>() as i64;
}

fn possible_solutions<'a>(
    field: &'a str,
    spring_group: &'a [usize],
    cache: &mut HashMap<(&'a str, &'a [usize]), usize>,
) -> usize {
    match cache.get(&(field, spring_group)) {
        Some(solutions) => {
            return *solutions;
        }
        None => {}
    }
    let contains_hash = field.contains("#");
    let contains_pos = field.contains("#") || field.contains("?");
    if spring_group.len() != 0 && !contains_pos {
        return 0;
    }
    if spring_group.len() == 0 && contains_hash {
        return 0;
    }
    if spring_group.len() == 0 && !contains_hash {
        return 1;
    }
    let need_len = spring_group[0];
    if need_len > field.len() {
        return 0;
    }
    let mut res = 0;
    if field.starts_with(".") || field.starts_with("?") {
        let possible_solutions = possible_solutions(&field[1..], spring_group, cache);
        cache.insert((&field[1..], spring_group), possible_solutions);
        res = res + possible_solutions;
    }
    if field.starts_with("?") {
        if !field[0..need_len].contains(".") {
            if need_len == field.len() {
                let possible_solutions =
                    possible_solutions(&field[need_len..], &spring_group[1..], cache);
                cache.insert((&field[need_len..], &spring_group[1..]), possible_solutions);
                res = res + possible_solutions;
            } else if need_len == field.len() - 1 {
                if field[need_len..need_len + 1].contains("#") {
                    return res;
                }
                let possible_solutions =
                    possible_solutions(&field[need_len + 1..], &spring_group[1..], cache);
                cache.insert(
                    (&field[need_len + 1..], &spring_group[1..]),
                    possible_solutions,
                );
                res = res + possible_solutions;
            } else {
                if field[need_len..need_len + 1].contains("#") {
                    return res;
                }
                let possible_solutions =
                    possible_solutions(&field[need_len + 1..], &spring_group[1..], cache);
                cache.insert(
                    (&field[need_len + 1..], &spring_group[1..]),
                    possible_solutions,
                );
                res = res + possible_solutions;
            }
        }
    }
    if field.starts_with("#") {
        if field[0..need_len].contains(".") {
            return 0;
        }
        if need_len == field.len() {
            let possible_solutions =
                possible_solutions(&field[need_len..], &spring_group[1..], cache);
            cache.insert((&field[need_len..], &spring_group[1..]), possible_solutions);
            res = res + possible_solutions;
        } else if need_len == field.len() - 1 {
            if field[need_len..need_len + 1].contains("#") {
                return res;
            }
            let possible_solutions =
                possible_solutions(&field[need_len + 1..], &spring_group[1..], cache);
            cache.insert(
                (&field[need_len + 1..], &spring_group[1..]),
                possible_solutions,
            );
            res = res + possible_solutions;
        } else {
            if field[need_len..need_len + 1].contains("#") {
                return res;
            }
            let possible_solutions =
                possible_solutions(&field[need_len + 1..], &spring_group[1..], cache);
            cache.insert(
                (&field[need_len + 1..], &spring_group[1..]),
                possible_solutions,
            );
            res = res + possible_solutions;
        }
    }

    return res;
}

pub fn part02(input: &str) -> i64 {
    return input
        .lines()
        .map(|l| {
            let (field, spring_group): (String, Vec<usize>) = l
                .split_once(" ")
                .map(|(field, r)| {
                    let spring_count: Vec<usize> = iter::repeat(
                        r.split(",")
                            .into_iter()
                            .map(|n| {
                                return n.parse::<usize>().expect("???");
                            })
                            .collect::<Vec<usize>>(),
                    )
                    .take(5)
                    .flatten()
                    .collect();
                    (vec![field; 5].join("?"), spring_count)
                })
                .expect("split ?");
            // let field_pieces: Vec<&str> = field
            //     .split(".")
            //     .filter(|w| {
            //         return w.len() != 0;
            //     })
            //     .collect();
            let total = possible_solutions(&field, &spring_group, &mut HashMap::new());
            return total;
            // return 0;
        })
        .sum::<usize>() as i64;
}
