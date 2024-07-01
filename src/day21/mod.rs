use std::collections::HashSet;

const ROCK: &str = "#";

pub fn part01(input: &str) -> Option<i64> {
    let mut start: Option<(usize, usize)> = Option::None;
    let map: Vec<Vec<&str>> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let line: Vec<&str> = l
                .split("")
                .into_iter()
                .enumerate()
                .map(|(j, c)| {
                    if c == "S" {
                        start = Some((i, j));
                    }
                    return c;
                })
                .collect();
            return line;
        })
        .collect();
    if start.is_none() {
        panic!("no start");
    }
    let s = start.unwrap();
    let total_step = 64;
    let moves = get_move(s, total_step, map);

    return Some(moves as i64);
}

fn get_move(s: (usize, usize), total_step: i32, map: Vec<Vec<&str>>) -> usize {
    let mut even_pos: HashSet<(usize, usize)> = HashSet::new();
    let mut odd_pos: HashSet<(usize, usize)> = HashSet::new();
    let mut start_pos = Vec::new();
    start_pos.push(s);
    for step in 0..total_step as usize {
        if start_pos.len() == 0 {
            break;
        }
        // print_map(step, &map, &even_pos, &odd_pos);
        let next_step_pos = walk(&start_pos, &map, step, &mut even_pos, &mut odd_pos);
        // println!("{:?}", start_pos);
        start_pos = next_step_pos;
    }
    let moves = odd_pos.len();
    moves
}

fn print_map(
    step: usize,
    map: &Vec<Vec<&str>>,
    even_pos: &HashSet<(usize, usize)>,
    odd_pos: &HashSet<(usize, usize)>,
) {
    let n = step % 2;
    map.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, s)| {
            if n == 1 {
                if even_pos.contains(&(i, j)) {
                    print!(" {}", "O");
                } else {
                    print!(" {}", s);
                }
            }
            if n == 0 {
                if odd_pos.contains(&(i, j)) {
                    print!(" {}", "O");
                } else {
                    print!(" {}", s);
                }
            }
        });
        println!("")
    });
    println!("---------------------------");
}

fn walk(
    start_pos: &Vec<(usize, usize)>,
    map: &Vec<Vec<&str>>,
    step: usize,
    even_pos: &mut HashSet<(usize, usize)>,
    odd_pos: &mut HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut next_step_pos = Vec::new();
    for s in start_pos {
        let neighbors = get_legal_neighbors(*s, map);
        let n = step % 2;
        for neighbor in neighbors {
            if n == 0 && !even_pos.contains(&neighbor) {
                even_pos.insert(neighbor);
                next_step_pos.push(neighbor);
            }
            if n == 1 && !odd_pos.contains(&neighbor) {
                odd_pos.insert(neighbor);
                next_step_pos.push(neighbor);
            }
        }
    }
    next_step_pos
}

fn get_legal_neighbors(s: (usize, usize), map: &Vec<Vec<&str>>) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if s.0 > 0 && map[s.0 - 1][s.1] != ROCK {
        neighbors.push((s.0 - 1, s.1));
    }
    if s.0 + 1 < map.len() && map[s.0 + 1][s.1] != ROCK {
        neighbors.push((s.0 + 1, s.1));
    }
    if s.1 > 0 && map[s.0][s.1 - 1] != ROCK {
        neighbors.push((s.0, s.1 - 1));
    }
    if s.1 + 1 < map[0].len() && map[s.0][s.1 + 1] != ROCK {
        neighbors.push((s.0, s.1 + 1));
    }
    neighbors
}

pub fn part02(input: &str) -> Option<i64> {
    return Some(0);
}
