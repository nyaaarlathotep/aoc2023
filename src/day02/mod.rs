use std::collections::HashMap;

pub fn part01(input: &str) -> i64 {
    let mut limit: HashMap<&str, i64> = HashMap::with_capacity(3);
    limit.insert("red", 12);
    limit.insert("green", 13);
    limit.insert("blue", 14);
    let a: i64 = input
        .lines()
        .enumerate()
        .map(|(i, line)| {

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
    return -1;
}
