use std::cmp::min;

pub fn part01(input: &str) -> i64 {
    return input
        .split("\n\n")
        .map(|part| {
            let mirror: Vec<&[u8]> = part.lines().map(|l| l.as_bytes()).collect::<Vec<&[u8]>>();

            let mirror_width = mirror.len();
            let mirror_height = mirror[0].len();
            for i in 0..mirror_width - 1 {
                let mut mirroralbe = true;
                for j in 0..mirror_height {
                    if !mirroralbe {
                        break;
                    }
                    for len in 1..i {
                        println!("{:?}-{:?}:{:?}", j, i-len+1, mirror.get(j).expect("?")[i] as char);
                        println!(
                            "{:?}-{:?}:{:?}",
                            j,
                            i + len,
                            mirror.get(j).expect("?")[i + len] as char
                        );
                        println!("");
                        if mirror.get(j).expect("?")[i - len + 1]
                            != mirror.get(j).expect("???")[i + len]
                        {
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
                let mut mirroralbe = true;
                for j in 0..mirror_width {
                    if !mirroralbe {
                        break;
                    }
                    for len in 1..i {
                        println!(
                            "{:?}-{:?}:{:?}",
                            i - len + 1,
                            j,
                            mirror.get(i - len + 1).expect("?")[j] as char
                        );
                        println!(
                            "{:?}-{:?}:{:?}",
                            i + len,
                            j,
                            mirror.get(i + len).expect("?")[j] as char
                        );
                        println!("");
                        if mirror.get(i - len + 1).expect("?")[j]
                            != mirror.get(i + len).expect("???")[j]
                        {
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
