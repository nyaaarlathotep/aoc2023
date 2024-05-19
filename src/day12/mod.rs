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
            let total = possible_solutions(field, &spring_group);
            return total;
        })
        .sum::<usize>() as i64;
}

fn possible_solutions(field: &str, spring_group: &[usize]) -> usize {
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
        res = res + possible_solutions(&field[1..], spring_group);
    }
    if field.starts_with("?") {
        if !field[0..need_len].contains(".") {
            if need_len == field.len() {
                res = res + possible_solutions(&field[need_len..], &spring_group[1..]);
            } else if need_len == field.len() - 1 {
                if field[need_len..need_len + 1].contains("#") {
                    return res;
                }
                res = res + possible_solutions(&field[need_len + 1..], &spring_group[1..]);
            } else {
                if field[need_len..need_len + 1].contains("#") {
                    return res;
                }
                res = res + possible_solutions(&field[need_len + 1..], &spring_group[1..]);
            }
        }
    }
    if field.starts_with("#") {
        if field[0..need_len].contains(".") {
            return 0;
        }
        if need_len == field.len() {
            res = res + possible_solutions(&field[need_len..], &spring_group[1..]);
        } else if need_len == field.len() - 1 {
            if field[need_len..need_len + 1].contains("#") {
                return res;
            }
            res = res + possible_solutions(&field[need_len + 1..], &spring_group[1..]);
        } else {
            if field[need_len..need_len + 1].contains("#") {
                return res;
            }
            res = res + possible_solutions(&field[need_len + 1..], &spring_group[1..]);
        }
    }

    return res;
}


pub fn part02(input: &str) -> i64 {
    return 0;
}
