pub fn part01(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let (field, spring_group): (Vec<char>, Vec<usize>) = l
                .split_once(" ")
                .map(|(field, r)| {
                    (
                        field.chars().collect(),
                        r.split(",")
                            .into_iter()
                            .map(|n| {
                                return n.parse::<usize>().ok().unwrap();
                            })
                            .collect(),
                    )
                })
                .unwrap();
            let total = possible_solutions(&field, &spring_group);
            return total;
        })
        .sum::<usize>();
    return 0;
}

fn possible_solutions(field: &[char], spring_group: &[usize]) -> usize {
    if spring_group.len() == 0 {
        return 1;
    }
    let mut index = 0;
    let need_contain_spring_num = spring_group[0];
    let mut spring_count_now = 0;
    loop {
        if index >= field.len() {
            return 0;
        }
        if field[index] == '.' {
            spring_count_now = 0;
        }
        if field[index] == '?' || field[index] == '#' {
            spring_count_now = spring_count_now + 1;
        }
        if spring_count_now == need_contain_spring_num {
            if index + 1 >= field.len() || spring_group.len() == 1 {
                return 1;
            }
            let current_possible = 1 + possible_solutions(&field[index + 2..], &spring_group[1..]);
            let next_solutions = possible_solutions(&field[1..], spring_group);
            return current_possible + next_solutions;
        }
        index = index + 1;
    }
}

pub fn part02(input: &str) -> i64 {
    return 0;
}
