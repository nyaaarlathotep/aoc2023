use std::cmp::min;

pub fn part01(input: &str) -> i64 {
    return input
        .split("\n\n")
        .map(|part| {
            let mirror: Vec<&[u8]> = part.lines().map(|l| l.as_bytes()).collect::<Vec<&[u8]>>();

            let mirror_width = mirror[0].len();
            let mirror_height = mirror.len();
            for i in 0..mirror_width - 1 {
                let mut mirroralbe = true;
                let max_mirror_len = min(i + 1, mirror_width - i - 1);
                // println!("i + 1:{}", i + 1);
                // println!("mirror_width - i - 1:{}", mirror_width - i - 1);
                // println!("maxLen:{}", max_mirror_len);
                for j in 0..mirror_height {
                    if !mirroralbe {
                        break;
                    }
                    for len in 0..max_mirror_len {
                        let left_char = mirror.get(j).expect("?")[i - len];
                        let right_char = mirror.get(j).expect("???")[i + 1 + len];
                        // println!("{:?}-{:?}:{:?}", j, i - len, left_char);
                        // println!("{:?}-{:?}:{:?}", j, i + 1 + len, right_char);
                        // println!("");
                        if left_char != right_char {
                            mirroralbe = false;
                            break;
                        }
                    }
                }
                if mirroralbe {
                    return i + 1;
                }
            }
            for i in 0..mirror_height - 1 {
                let max_mirror_len = min(i + 1, mirror_height - i - 1);
                let mut mirroralbe = true;
                // println!("maxLen:{}", max_mirror_len);
                for j in 0..mirror_width {
                    if !mirroralbe {
                        break;
                    }
                    for len in 0..max_mirror_len {
                        let left_num = mirror.get(i - len).expect("?")[j];
                        let right_num = mirror.get(i + len + 1).expect("???")[j];
                        // println!("{:?}-{:?}:{:?}", i - len, j, left_num);
                        // println!("{:?}-{:?}:{:?}", i + 1 + len, j, right_num);
                        // println!("");
                        if left_num != right_num {
                            mirroralbe = false;
                            break;
                        }
                    }
                }
                if mirroralbe {
                    return 100 * (i + 1);
                }
            }
            panic!("????");
        })
        .sum::<usize>() as i64;
}

pub fn part02(input: &str) -> i64 {
    return 0;
}
