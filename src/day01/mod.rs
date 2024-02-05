use std::collections::HashMap;

pub fn part01(input: &str) -> i64 {
    let mut total: i64 = 0;
    let mut first: i64 = -1;
    let mut last: i64 = -1;
    let mut count = 0;
    for r in input.chars() {
        if r == '\n' {
            total = total + first * 10 + last;
            first = -1;
            last = -1;
            count = count + 1;
        }
        if r >= '0' && r <= '9' {
            if first == -1 {
                first = r as i64 - '0' as i64;
            }

            last = r as i64 - '0' as i64;
        }
    }
    println!("count: {:?}", count);
    return total;
}

pub fn part02(input: &str) -> i64 {
    let mut number_map: HashMap<&str, i64> = HashMap::new();

    number_map.insert("one", 1);
    number_map.insert("two", 2);
    number_map.insert("three", 3);
    number_map.insert("four", 4);
    number_map.insert("five", 5);
    number_map.insert("six", 6);
    number_map.insert("seven", 7);
    number_map.insert("eight", 8);
    number_map.insert("nine", 9);
    number_map.insert("1", 1);
    number_map.insert("2", 2);
    number_map.insert("3", 3);
    number_map.insert("4", 4);
    number_map.insert("5", 5);
    number_map.insert("6", 6);
    number_map.insert("7", 7);
    number_map.insert("8", 8);
    number_map.insert("9", 9);
    let total: i64 = input
        .lines()
        .filter_map(|line| {
            let first = (0..line.len()).find_map(|start| {
                number_map.iter().find_map(|(str, val)| {
                    if line[start..].starts_with(str) {
                        return Some(val);
                    } else {
                        return None;
                    }
                })
            });
            let last = (0..line.len()).rev().find_map(|end| {
                number_map.iter().find_map(|(str, val)| {
                    if line[..=end].ends_with(str) {
                        return Some(val);
                    } else {
                        return None;
                    }
                })
            });
            first.zip(last).map(|(a, b)| {
                return a * 10 + b;
            })
        })
        .sum();
    println!("{:?}", total);
    return total;
}