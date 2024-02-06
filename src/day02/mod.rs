use std::collections::HashMap;

pub fn part01(input: &str) -> i64 {
    let mut limit: HashMap<&str, i64> = HashMap::with_capacity(3);
    limit.insert("red", 12);
    limit.insert("green", 13);
    limit.insert("blue", 14);
    let f = |i: usize, line: &str| {
        let game = line.split_once(":").unwrap();
        let mut balls = game.1.split(";");
        let illeagal = balls.any(|str| {
            let pass = str.split(",").any(|ball| {
                let num_color_tuple = ball[1..].split_once(" ").unwrap();
                return legal(&limit, num_color_tuple);
            });
            return pass;
        });
        if illeagal {
            return 0;
        }
        return (i + 1) as i64;
    };
    let a: i64 = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            return f(i, line);
        })
        .sum();
    return a;
}

fn legal(limit: &HashMap<&str, i64>, num_color_tuple: (&str, &str)) -> bool {
    let illegal_color = limit.iter().any(|(&str, &val)| {
        if str.eq(num_color_tuple.1) {
            let num: i64 = num_color_tuple.0.parse().unwrap();
            return num > val;
        }
        return false;
    });
    return illegal_color;
}

pub fn part02(input: &str) -> i64 {
    let f = |_: usize, line: &str| {
        let mut max_map: HashMap<&str, i64> = HashMap::with_capacity(3);
        let game = line.split_once(":").unwrap();
        let balls = game.1.split(";");
        balls.for_each(|str| {
            str.split(",").for_each(|ball| {
                let num_color_tuple = ball[1..].split_once(" ").unwrap();
                let num: i64 = num_color_tuple.0.parse().unwrap();
                match max_map.get(num_color_tuple.1) {
                    Some(&num_max) => {
                        if num_max < num {
                            max_map.insert(num_color_tuple.1, num);
                        }
                    }
                    None => {
                        max_map.insert(num_color_tuple.1, num);
                    }
                };
            });
        });

        let total = max_map.iter().fold(1, |acc, entry| {
            return acc * entry.1;
        });
        return total;
    };
    let a: i64 = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            return f(i, line);
        })
        .sum();
    return a;
}
