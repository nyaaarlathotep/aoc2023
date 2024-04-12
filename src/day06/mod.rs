use itertools::Itertools;

pub fn part01(input: &str) -> i64 {
    input
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
        })
        .unwrap() as i64
}

pub fn left_start(time: u64, dis: u64, left: u64, right: u64) -> u64 {
    let mid = (left + right) / 2;
    if mid * (time - mid) > dis && (mid - 1) * (time - (mid - 1)) < dis {
        return mid;
    }
    if mid * (time - mid) > dis {
        return left_start(time, dis, left, mid);
    }

    left_start(time, dis, mid, right)
}

pub fn right_start(time: u64, dis: u64, left: u64, right: u64) -> u64 {
    let mid = (left + right) / 2;
    if mid * (time - mid) > dis && (mid + 1) * (time - (mid + 1)) < dis {
        return mid;
    }
    if mid * (time - mid) > dis {
        return right_start(time, dis, mid, right);
    }

    right_start(time, dis, left, mid)
}

pub fn part02(input: &str) -> i64 {
    input
        .split_once("\n")
        .map(|(t, d)| {
            let time = t
                .split_once(':')
                .unwrap()
                .1
                .chars()
                .filter_map(|ch| ch.to_digit(10).map(u64::from))
                .reduce(|acc, d| acc * 10 + d)
                .unwrap();

            let dis = d
                .split_once(':')
                .unwrap()
                .1
                .chars()
                .filter_map(|ch| ch.to_digit(10).map(u64::from))
                .reduce(|acc, d| acc * 10 + d)
                .unwrap();
            (time, dis)
        })
        .map(|(time, dis)| {
            let l = left_start(time, dis, 0, time / 2);
            let r = right_start(time, dis, time / 2, time);
            r - l + 1
        })
        .unwrap() as i64
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TimeRecord {
    times: Vec<u32>,
    distances: Vec<u32>,
}
