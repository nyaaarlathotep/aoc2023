pub fn part01(input: &str) -> i64 {
    let a = input
        .lines()
        .into_iter()
        .map(|l| {
            let nums: Vec<_> = l
                .split(" ")
                .into_iter()
                .map(|raw_num| {
                    return (raw_num.parse::<i64>()).ok().unwrap();
                })
                .collect();
            let s = get_next(nums);
            return s;
        })
        .sum::<i64>();
    return a;
}

fn get_next(nums: Vec<i64>) -> i64 {
    let mut all_zero = true;
    for num in &nums {
        if *num != 0 {
            all_zero = false;
            break;
        }
    }
    if all_zero {
        return 0;
    }
    let mut next_row: Vec<i64> = Vec::new();
    for i in 0..(nums.len() - 1) {
        next_row.push(nums[i + 1] - nums[i]);
    }

    return nums[nums.len() - 1] + get_next(next_row);
}

pub fn part02(input: &str) -> i64 {
    return 0;
}
