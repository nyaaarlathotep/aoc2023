use std::collections::{HashMap, HashSet};

pub fn part01(input: &str) -> i64 {
    let mut pipes: HashMap<(i64, i64), Pipe> = HashMap::new();

    let mut start: Pipe = Pipe {
        row: 0,
        col: 0,
        symbol: '?',
        steps: 0,
    };
    input.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            match c {
                '.' => {}
                'S' => {
                    let s = Pipe {
                        row: i as i64,
                        col: j as i64,
                        symbol: c,
                        steps: 0,
                    };
                    start = s;
                    pipes.insert((i as i64, j as i64), s);
                }
                default => {
                    pipes.insert(
                        (i as i64, j as i64),
                        Pipe {
                            row: i as i64,
                            col: j as i64,
                            symbol: default,
                            steps: usize::MAX,
                        },
                    );
                }
            };
        });
    });
    let mut next_step: Vec<Pipe> = Vec::new();
    match pipes.get_mut(&(start.row - 1, start.col)) {
        Some(p) => {
            if p.symbol == '|' || p.symbol == '7' || p.symbol == 'F' {
                p.steps = 1;
                next_step.push(*p);
            }
        }
        _ => {}
    }
    match pipes.get_mut(&(start.row + 1, start.col)) {
        Some(p) => {
            if p.symbol == '|' || p.symbol == 'J' || p.symbol == 'L' {
                p.steps = 1;
                next_step.push(*p)
            }
        }
        _ => {}
    }
    match pipes.get_mut(&(start.row, start.col - 1)) {
        Some(p) => {
            if p.symbol == '-' || p.symbol == 'L' || p.symbol == 'F' {
                p.steps = 1;
                next_step.push(*p)
            }
        }
        _ => {}
    }
    match pipes.get_mut(&(start.row, start.col + 1)) {
        Some(p) => {
            if p.symbol == '-' || p.symbol == '7' || p.symbol == 'J' {
                p.steps = 1;
                next_step.push(*p)
            }
        }
        _ => {}
    }

    let mut step_now = 1;
    loop {
        (next_step, pipes) = get_biggest_step(&next_step, pipes, step_now);
        if next_step.len() == 0 {
            return step_now as i64;
        }
        step_now = step_now + 1;
    }
}

fn get_biggest_step(
    first_step: &Vec<Pipe>,
    mut pipes: HashMap<(i64, i64), Pipe>,
    step_now: usize,
) -> (Vec<Pipe>, HashMap<(i64, i64), Pipe>) {
    let mut next_step: Vec<Pipe> = Vec::new();
    for p in first_step {
        match p.symbol {
            '|' => {
                match pipes.get_mut(&(p.row - 1, p.col)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                            next_step.push(*next_p);
                        }
                    }
                    _ => {}
                };
                match pipes.get_mut(&(p.row + 1, p.col)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                            next_step.push(*next_p);
                        }
                    }
                    _ => {}
                };
            }
            '-' => {
                match pipes.get_mut(&(p.row, p.col - 1)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                            next_step.push(*next_p);
                        }
                    }
                    _ => {}
                };
                match pipes.get_mut(&(p.row, p.col + 1)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                            next_step.push(*next_p);
                        }
                    }
                    _ => {}
                };
            }
            '7' => {
                match pipes.get_mut(&(p.row + 1, p.col)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                            next_step.push(*next_p);
                        }
                    }
                    _ => {}
                };
                match pipes.get_mut(&(p.row, p.col - 1)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                            next_step.push(*next_p);
                        }
                    }
                    _ => {}
                };
            }
            'F' => {
                match pipes.get_mut(&(p.row + 1, p.col)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                            next_step.push(*next_p);
                        }
                    }
                    _ => {}
                };
                match pipes.get_mut(&(p.row, p.col + 1)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                            next_step.push(*next_p);
                        }
                    }
                    _ => {}
                };
            }
            'J' => {
                match pipes.get_mut(&(p.row - 1, p.col)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                            next_step.push(*next_p);
                        }
                    }
                    _ => {}
                };
                match pipes.get_mut(&(p.row, p.col - 1)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                            next_step.push(*next_p);
                        }
                    }
                    _ => {}
                };
            }
            'L' => {
                match pipes.get_mut(&(p.row - 1, p.col)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                            next_step.push(*next_p);
                        }
                    }
                    _ => {}
                };
                match pipes.get_mut(&(p.row, p.col + 1)) {
                    Some(next_p) => {
                        if next_p.steps == usize::MAX {
                            next_p.steps = step_now;
                            next_step.push(*next_p);
                        }
                    }
                    _ => {}
                };
            }
            default => {
                panic!("???{:?}", default)
            }
        }
    }
    return (next_step, pipes);
    // if next_step.len() == 0 {
    //     return Some((step_now - 1) as i64);
    // }
    // return get_biggest_step(next_step, pipes, step_now + 1);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pipe {
    row: i64,
    col: i64,
    symbol: char,
    steps: usize,
}

pub fn part02(input: &str) -> i64 {
    let mut pipes: HashMap<(i64, i64), Pipe> = HashMap::new();

    let mut start: Pipe = Pipe {
        row: 0,
        col: 0,
        symbol: '?',
        steps: 0,
    };
    input.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            match c {
                '.' => {}
                'S' => {
                    let s = Pipe {
                        row: i as i64,
                        col: j as i64,
                        symbol: c,
                        steps: 0,
                    };
                    start = s;
                    pipes.insert((i as i64, j as i64), s);
                }
                default => {
                    pipes.insert(
                        (i as i64, j as i64),
                        Pipe {
                            row: i as i64,
                            col: j as i64,
                            symbol: default,
                            steps: usize::MAX,
                        },
                    );
                }
            };
        });
    });
    let mut next_step: Vec<Pipe> = Vec::new();
    match pipes.get_mut(&(start.row - 1, start.col)) {
        Some(p) => {
            if p.symbol == '|' || p.symbol == '7' || p.symbol == 'F' {
                p.steps = 1;
                next_step.push(*p);
            }
        }
        _ => {}
    }
    match pipes.get_mut(&(start.row + 1, start.col)) {
        Some(p) => {
            if p.symbol == '|' || p.symbol == 'J' || p.symbol == 'L' {
                p.steps = 1;
                next_step.push(*p)
            }
        }
        _ => {}
    }
    match pipes.get_mut(&(start.row, start.col - 1)) {
        Some(p) => {
            if p.symbol == '-' || p.symbol == 'L' || p.symbol == 'F' {
                p.steps = 1;
                next_step.push(*p)
            }
        }
        _ => {}
    }
    match pipes.get_mut(&(start.row, start.col + 1)) {
        Some(p) => {
            if p.symbol == '-' || p.symbol == '7' || p.symbol == 'J' {
                p.steps = 1;
                next_step.push(*p)
            }
        }
        _ => {}
    }

    let mut main_pipe: HashSet<(i64, i64)> = HashSet::new();
    let mut step_now = 1;
    loop {
        for p in &next_step {
            main_pipe.insert((p.row, p.col));
        }
        (next_step, pipes) = get_biggest_step(&next_step, pipes, step_now);
        if next_step.len() == 0 {
            break;
        }
        step_now = step_now + 1;
    }
    let mut inner_poses: Vec<(usize, usize)> = Vec::new();

    main_pipe.insert((start.row, start.col));
    input.lines().enumerate().for_each(|(i, l)| {
        let mut inner = false;
        let mut inner_poses_line: Vec<(usize, usize)> = Vec::new();
        l.chars().enumerate().for_each(|(j, c)| {
            if main_pipe.contains(&(i as i64, j as i64)) {
                if c == '|' || c == 'F' || c == '7' {
                    inner = !inner;
                    inner_poses.append(&mut inner_poses_line);
                }
            } else if inner {
                inner_poses_line.push((i, j));
            }
        });
        inner_poses_line.clear();
    });
    // print_pipe(input, &inner_poses, main_pipe);

    return inner_poses.len() as i64;
}

fn print_pipe(input: &str, inner_poses: &Vec<(usize, usize)>, main_pipe: HashSet<(i64, i64)>) {
    input.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if inner_poses.contains(&(i, j)) {
                print!("+ ");
            } else if main_pipe.contains(&(i as i64, j as i64)) {
                print!("{:} ", c)
            } else {
                print!("O ");
            }
        });
        println!()
    });
}
