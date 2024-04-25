use std::{collections::HashMap, default, fs};

pub fn part01(input: &str) -> i64 {
    let mut pipes: HashMap<(usize, usize), Pipe> = HashMap::new();

    let start: Pipe = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let s = l
                .chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => None,
                    'S' => {
                        let s = Pipe {
                            row: i,
                            col: j,
                            symbol: c,
                            steps: 0,
                        };
                        pipes.insert((i, j), s);
                        return Some(s);
                    }
                    default => {
                        pipes.insert(
                            (i, j),
                            Pipe {
                                row: i,
                                col: j,
                                symbol: default,
                                steps: usize::MAX,
                            },
                        );
                        return None;
                    }
                })
                .find(|s| s.is_some())
                .unwrap();
            return s;
        })
        .find(|s| s.is_some())
        .unwrap()
        .unwrap();
    let mut first_step: Vec<Pipe> = Vec::new();
    match pipes.get_mut(&(start.row - 1, start.col)) {
        Some(p) => {
            if p.symbol == '|' || p.symbol == '7' || p.symbol == 'F' {
                p.steps = 1;
                first_step.push(*p);
            }
        }
        _ => {}
    }
    match pipes.get_mut(&(start.row + 1, start.col)) {
        Some(p) => {
            if p.symbol == '|' || p.symbol == 'J' || p.symbol == 'L' {
                p.steps = 1;
                first_step.push(*p)
            }
        }
        _ => {}
    }
    match pipes.get_mut(&(start.row, start.col - 1)) {
        Some(p) => {
            if p.symbol == '-' || p.symbol == 'L' || p.symbol == 'F' {
                p.steps = 1;
                first_step.push(*p)
            }
        }
        _ => {}
    }
    match pipes.get_mut(&(start.row, start.col + 1)) {
        Some(p) => {
            if p.symbol == '-' || p.symbol == '7' || p.symbol == 'J' {
                p.steps = 1;
                first_step.push(*p)
            }
        }
        _ => {}
    }

    let step_now = 2;
    let mut next_step: Vec<Pipe> = Vec::new();
    for p in first_step {
        match p.symbol {
            '|' => {
                match pipes.get_mut(&(p.row - 1, p.col)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                        }
                        next_step.push(*next_p);
                    }
                    _ => {}
                };
                match pipes.get_mut(&(p.row + 1, p.col)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                        }
                        next_step.push(*next_p);
                    }
                    _ => {}
                };
            }
            '-' => {
                match pipes.get_mut(&(p.row, p.col - 1)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                        }
                        next_step.push(*next_p);
                    }
                    _ => {}
                };
                match pipes.get_mut(&(p.row, p.col + 1)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                        }
                        next_step.push(*next_p);
                    }
                    _ => {}
                };
            }
            '7' => {
                match pipes.get_mut(&(p.row + 1, p.col)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                        }
                        next_step.push(*next_p);
                    }
                    _ => {}
                };
                match pipes.get_mut(&(p.row, p.col - 1)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                        }
                        next_step.push(*next_p);
                    }
                    _ => {}
                };
            }
            'F' => {
                match pipes.get_mut(&(p.row + 1, p.col)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                        }
                        next_step.push(*next_p);
                    }
                    _ => {}
                };
                match pipes.get_mut(&(p.row, p.col + 1)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                        }
                        next_step.push(*next_p);
                    }
                    _ => {}
                };
            }
            'J' => {
                match pipes.get_mut(&(p.row - 1, p.col)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                        }
                        next_step.push(*next_p);
                    }
                    _ => {}
                };
                match pipes.get_mut(&(p.row, p.col - 1)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                        }
                        next_step.push(*next_p);
                    }
                    _ => {}
                };
            }
            'L' => {
                match pipes.get_mut(&(p.row - 1, p.col)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                        }
                        next_step.push(*next_p);
                    }
                    _ => {}
                };
                match pipes.get_mut(&(p.row, p.col + 1)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                        }
                        next_step.push(*next_p);
                    }
                    _ => {}
                };
            }
            default => {
                panic!("???{:?}", default)
            }
        }
    }
    if next_step.len()==0 {
        return step_now as i64;
    }
    return 0;
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pipe {
    row: usize,
    col: usize,
    symbol: char,
    steps: usize,
}

pub fn part02(input: &str) -> i64 {
    return 0;
}
