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
