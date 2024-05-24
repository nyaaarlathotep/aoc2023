use core::panic;
use std::cmp::min;

enum MirrorType {
    vertical,
    horizon,
}

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
                for j in 0..mirror_height {
                    if !mirroralbe {
                        break;
                    }
                    for len in 0..max_mirror_len {
                        let left_char = mirror.get(j).expect("?")[i - len];
                        let right_char = mirror.get(j).expect("???")[i + 1 + len];
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
                for j in 0..mirror_width {
                    if !mirroralbe {
                        break;
                    }
                    for len in 0..max_mirror_len {
                        let left_num = mirror.get(i - len).expect("?")[j];
                        let right_num = mirror.get(i + len + 1).expect("???")[j];
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
    return input
        .split("\n\n")
        .map(|part| {
            let mirror: Vec<&[u8]> = part.lines().map(|l| l.as_bytes()).collect::<Vec<&[u8]>>();

            let mirror_width = mirror[0].len();
            let mirror_height = mirror.len();
            for i in 0..mirror_width - 1 {
                let max_mirror_len = min(i + 1, mirror_width - i - 1);
                let mut miss_count = 0;
                for j in 0..mirror_height {
                    for len in 0..max_mirror_len {
                        let left_char = mirror.get(j).expect("?")[i - len];
                        let right_char = mirror.get(j).expect("???")[i + 1 + len];
                        if left_char != right_char {
                            miss_count = miss_count + 1;
                            if miss_count > 1 {
                                break;
                            }
                        }
                    }
                }
                if miss_count == 1 {
                    return i + 1;
                }
            }
            for i in 0..mirror_height - 1 {
                let max_mirror_len = min(i + 1, mirror_height - i - 1);

                let mut miss_count = 0;
                for j in 0..mirror_width {
                    for len in 0..max_mirror_len {
                        let left_char = mirror.get(i - len).expect("?")[j];
                        let right_char = mirror.get(i + len + 1).expect("???")[j];

                        if left_char != right_char {
                            miss_count = miss_count + 1;
                            if miss_count > 1 {
                                break;
                            }
                        }
                    }
                }
                if miss_count == 1 {
                    return 100 * (i + 1);
                }
            }
            panic!("11111");
        })
        .sum::<usize>() as i64;
}
