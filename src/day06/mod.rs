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
            r.times.iter().enumerate().map(|(t, &time)| {
                let distance = r.distances.get(t).unwrap();
                let total_time = time;
                (0..time).map(|wait_time| wait_time)
            });
        });

    return 0;
}

pub fn part02(input: &str) -> i64 {
    return 0;
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TimeRecord {
    times: Vec<u32>,
    distances: Vec<u32>,
}
