pub fn part01(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let (filed, spring_group) = l
                .split_once(" ")
                .map(|(fielld, r)| {
                    (fielld, r
                        .split(" ")
                        .into_iter()
                        .map(|n| {
                            return n.parse::<usize>().ok().unwrap();
                        })
                        .collect())
                })
                .unwrap();
            fun_name(filed, spring_group);
            return 1;
        })
        .sum::<i64>();
    return 0;
}

fn fun_name(filed: &str, spring_group: Vec<usize>) -> usize {
    if spring_group.len()==0 {
        return 1;
    }
    
    return 1;
}

pub fn part02(input: &str) -> i64 {
    return 0;
}
