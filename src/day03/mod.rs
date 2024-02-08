use std::char;

pub fn part01(input: &str) -> i64 {
    let mut num_total_now: Option<u32> = None;
    let mut index_start: Option<usize> = None;
    let num_pic: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut total_res: i64 = 0;

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().into_iter().enumerate() {
            let is_num: bool;
            if let Some(num) = char.to_digit(10) {
                is_num = true;
                num_total_now = Some(num_total_now.unwrap_or(0) * 10 + num);
                if index_start.is_none() {
                    index_start = Some(col);
                }
            } else {
                is_num =false;
            }
            if !is_num || col == line.len() - 1 {
                if let Some((i_start, num)) = index_start.zip(num_total_now) {
                    let end = if is_num { col } else { col - 1 };
                    let operation = get_operation(row, i_start, end, &num_pic);
                    if operation.is_some() {
                        // println!("add:{:?}", num);
                        total_res = total_res + num as i64;
                    }
                }
                num_total_now = None;
                index_start = None;
            }
        }
    }
    return total_res;
}

fn get_operation(row: usize, i_start: usize, end: usize, num_pic: &Vec<Vec<char>>) -> Option<char> {
    for i in row.saturating_sub(1)..=row.saturating_add(1) {
        for j in i_start.saturating_sub(1)..=end.saturating_add(1) {
            if let Some(operation) = num_pic
                .get(i)
                .and_then(|checking_line| checking_line.get(j))
            {
                if !operation.is_numeric() && *operation != ('.') {
                    // println!("find operation:{:?},{:?}-->{:?}", i, j, operation);
                    return Some(*operation);
                }
            }
        }
    }
    return None;
}

pub fn part02(input: &str) -> i64 {
    return -1;
}
