
const EMPTY_VEC: Option<Vec<Len>> = None;
pub fn part01(input: &str) -> Option<i64> {
    let res: usize = input
        .split(",")
        .into_iter()
        .map(|str| {
            let current_value = str_hash(str);
            return current_value;
        })
        .sum();
    return Some(res as i64);
}

fn str_hash(str: &str) -> usize {
    let mut current_value: usize = 0;
    for ch in str.as_bytes() {
        current_value = current_value + *ch as usize;
        current_value = current_value * 17;
        current_value = current_value % 256;
    }
    current_value
}

pub fn part02(input: &str) -> Option<i64> {
    let mut boxes: [Option<Vec<Len>>; 256] = [EMPTY_VEC; 256];
    input.split(",").into_iter().for_each(|l| {
        if l.contains("-") {
            let hash = str_hash(&l[..l.len() - 1]);
            match &mut boxes[hash] {
                Some(boxx) => {
                    if let Some(index) = boxx.iter().position(|len| *len.label == l[..l.len() - 1])
                    {
                        boxx.remove(index);
                    }
                }
                None => {}
            }
        } else if l.contains("=") {
            let line = l.split_once("=").unwrap();
            let label = line.0;
            let focal_len = line.1.parse::<usize>().expect("???");
            let hash = str_hash(label);
            let new_len = Len { focal_len, label };
            match &mut boxes[hash] {
                Some(boxx) => {
                    if let Some(index) = boxx.iter().position(|len| *len.label == *label) {
                        boxx.get_mut(index).unwrap().focal_len = focal_len;
                    } else {
                        boxx.push(new_len)
                    }
                }
                None => {
                    let mut new_box: Vec<_> = Vec::new();
                    new_box.push(new_len);
                    boxes[hash] = Some(new_box);
                }
            }
        }
    });

    let res: usize = boxes
        .into_iter()
        .enumerate()
        .map(|(i, opt_box)| match opt_box {
            Some(boox) => {
                let box_sum: usize = boox
                    .into_iter()
                    .enumerate()
                    .map(|(j, len)| {
                        return (i + 1) * (j + 1) * len.focal_len;
                    })
                    .sum();
                return box_sum;
            }
            None => {
                return 0;
            }
        })
        .sum();

    return Some(res as i64);
}
#[test]
fn temp_test() {
    println!("part 1 res: {:?}", part01("HASH"));
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Len<'a> {
    focal_len: usize,
    label: &'a str,
}

#[test]
fn hash_test() {
    let mut v = vec!["1", "2"];
    println!("hash res: {:?}", str_hash(""));
}
