pub fn part01(input: &str) -> Option<i64> {
    let res: usize = input
        .split(",")
        .into_iter()
        .map(|str| {
            // println!("{}", str);
            let mut current_value: usize = 0;
            for ch in str.as_bytes() {
                current_value = current_value + *ch as usize;
                current_value = current_value * 17;
                current_value = current_value % 256;
            }
            // println!("cv:{}", current_value);
            // println!();
            return current_value;
        })
        .sum();
    return Some(res as i64);
}

pub fn part02(input: &str) -> Option<i64> {
    return Some(0);
}
#[test]
fn temp_test() {
    println!("part 1 res: {:?}", part01("HASH"));
}
