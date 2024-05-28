pub fn part01(input: &str) -> i64 {
    let map: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let height = map.len();
    let length = map[0].len();
    let mut total = 0;
    for j in 0..height {
        let mut this_line = 0;
        let mut last_rock = 0;
        for i in 0..length {
            if map[i][j] == b'O' {
                this_line = this_line + height - last_rock;
                last_rock = last_rock + 1;
            } else if map[i][j] == b'#' {
                last_rock = i + 1;
            } else if map[i][j] == b'.' {
            }
        }
        total = total + this_line;
    }

    return total as i64;
}

pub fn part02(input: &str) -> i64 {
    let mut map: Vec<Vec<u8>> = input
        .lines()
        .map(|l| {
            let a: String = String::from(l);
            let v = a.into_bytes();
            return v;
        })
        .collect();
    let length = map.len();
    let height = map[0].len();
    map = rotate(map, length, height);
    map = rotate(map, length, height);
    map = rotate(map, length, height);
    return 0;
}

fn rotate(mut map: Vec<Vec<u8>>, length: usize, height: usize) -> Vec<Vec<u8>> {
    for j in 0..length {
        let mut next_valid_pos = 0;
        for i in 0..height {
            if map[i][j] == b'O' {
                if next_valid_pos != i {
                    map[i][j] = b'.';
                    map[next_valid_pos][j] = b'O';
                }
                next_valid_pos = next_valid_pos + 1;
            } else if map[i][j] == b'#' {
                next_valid_pos = i + 1;
            } else if map[i][j] == b'.' {
            }
        }
    }
    for i in 0..height {
        let mut next_valid_pos = 0;
        for j in 0..length {
            if map[i][j] == b'O' {
                if next_valid_pos != j {
                    map[i][j] = b'.';
                    map[i][next_valid_pos] = b'O';
                }
                next_valid_pos = next_valid_pos + 1;
            } else if map[i][j] == b'#' {
                next_valid_pos = j + 1;
            }
        }
    }
    for j in 0..length {
        let mut next_valid_pos = height - 1;
        for i in (0..height).rev() {
            if map[i][j] == b'O' {
                if next_valid_pos != i {
                    map[i][j] = b'.';
                    map[next_valid_pos][j] = b'O';
                }
                if next_valid_pos != 0 {
                    next_valid_pos = next_valid_pos - 1;
                }
            } else if map[i][j] == b'#' {
                if i != 0 {
                    next_valid_pos = i - 1;
                }
            }
        }
    }
    for i in 0..height {
        let mut next_valid_pos = length - 1;
        for j in (0..length).rev() {
            if map[i][j] == b'O' {
                if next_valid_pos != j {
                    map[i][j] = b'.';
                    map[i][next_valid_pos] = b'O';
                }
                if next_valid_pos != 0 {
                    next_valid_pos = next_valid_pos - 1;
                }
            } else if map[i][j] == b'#' {
                if j != 0 {
                    next_valid_pos = j - 1;
                }
            }
        }
    }
    return map;
}

fn print_map(map: &Vec<Vec<u8>>) {
    map.iter().for_each(|v| {
        for c in v {
            print!("{:}", *c as char);
        }
        println!("");
    });
    println!("");
    println!("");
}

#[test]
fn temp_test() {
    let mut a = vec![vec![2, 3], vec![3, 5]];
    println!("{:?}", a);
    a[1][1] = 3;
    println!("{:?}", a);
}
