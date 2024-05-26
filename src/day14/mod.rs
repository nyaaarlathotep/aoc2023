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
    return 0;
}
