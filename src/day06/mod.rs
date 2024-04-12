use itertools::Itertools;

pub fn part01(input: &str) -> i64 {
    let res = input
        .split_once("\n")
        .map(|(t, d)| TimeRecord {
            times: t
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect_vec(),
            distances: d
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect_vec(),
        })
        .map(|r| {
            println!("?");
            let a: usize = r
                .times
                .into_iter()
                .enumerate()
                .map(|(t, time)| {
                    let distance = r.distances.get(t).unwrap();
                    let count: usize = (0..time)
                        .filter(|wait_time| {
                            return wait_time * (time - wait_time) > *distance;
                        })
                        .count();
                    return count;
                })
                .product::<usize>();
            return a;
        }).unwrap() as i64;

    return res;
}

pub fn part02(input: &str) -> i64 {
    return 0;
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TimeRecord {
    times: Vec<u32>,
    distances: Vec<u32>,
}
