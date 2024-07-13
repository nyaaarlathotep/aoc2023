use itertools::Itertools;

const ROCK: u8 = b'#';
const PATH: u8 = b'.';
const VISITED: u8 = b'O';
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        return Coordinate { x, y };
    }
}

pub fn part01(input: &str) -> Option<i64> {
    let mut map: Vec<Vec<u8>> = input
        .lines()
        .map(|l| {
            let a: String = String::from(l);
            let v = a.into_bytes();
            return v;
        })
        .collect();
    let mut start = Option::None;
    for i in 0..(&map[0]).len() {
        if *&map[0][i] == b'.' {
            start = Some(Coordinate::new(0, i));
            break;
        }
    }
    let s = start.expect("!");
    let mut res: Vec<usize> = Vec::new();

    map[s.x][s.y] = VISITED;
    walk(&mut res, &mut map, 0, &s);

    return Some(res.into_iter().max().unwrap() as i64);
}

fn walk(end_steps: &mut Vec<usize>, map: &mut Vec<Vec<u8>>, steps: usize, pos: &Coordinate) {
    if pos.x == map.len() - 1 {
        end_steps.push(steps);
        // print_map(&map);
        return;
    }
    let neighbors = get_legal_neighbors(pos, map);
    for neighbor in neighbors {
        map[neighbor.0.x][neighbor.0.y] = VISITED;
        walk(end_steps, map, steps + neighbor.1, &neighbor.0);
        map[neighbor.0.x][neighbor.0.y] = PATH;
    }
}

fn get_legal_neighbors(s: &Coordinate, map: &Vec<Vec<u8>>) -> Vec<(Coordinate, usize)> {
    let mut neighbors = Vec::new();
    if s.x > 0 && map[s.x - 1][s.y] != ROCK && map[s.x - 1][s.y] != VISITED {
        if map[s.x - 1][s.y] == b'^' {
            neighbors.push((Coordinate::new(s.x - 2, s.y), 2));
        } else if map[s.x - 1][s.y] == PATH {
            neighbors.push((Coordinate::new(s.x - 1, s.y), 1));
        }
    }
    if s.x + 1 < map.len() && map[s.x + 1][s.y] != ROCK && map[s.x + 1][s.y] != VISITED {
        if map[s.x + 1][s.y] == b'v' {
            neighbors.push((Coordinate::new(s.x + 2, s.y), 2));
        } else if map[s.x + 1][s.y] == PATH {
            neighbors.push((Coordinate::new(s.x + 1, s.y), 1));
        }
    }
    if s.y > 0 && map[s.x][s.y - 1] != ROCK && map[s.x][s.y - 1] != VISITED {
        if map[s.x][s.y - 1] == b'<' {
            neighbors.push((Coordinate::new(s.x, s.y - 2), 2));
        } else if map[s.x][s.y - 1] == PATH {
            neighbors.push((Coordinate::new(s.x, s.y - 1), 1));
        }
    }
    if s.y + 1 < map[0].len() && map[s.x][s.y + 1] != ROCK && map[s.x][s.y + 1] != VISITED {
        if map[s.x][s.y + 1] == b'>' {
            neighbors.push((Coordinate::new(s.x, s.y + 2), 2));
        } else if map[s.x][s.y + 1] == PATH {
            neighbors.push((Coordinate::new(s.x, s.y + 1), 1));
        }
    }
    neighbors
}

struct MapNode {
    mark: u8,
    record: usize,
}
pub fn part02(input: &str) -> Option<i64> {
    let mut map: Vec<Vec<MapNode>> = input
        .lines()
        .map(|l| {
            let a: String = String::from(l);
            let v = a
                .into_bytes()
                .into_iter()
                .map(|u| {
                    return MapNode { mark: u, record: 0 };
                })
                .collect_vec();
            return v;
        })
        .collect();
    let mut start = Option::None;
    for i in 0..(&map[0]).len() {
        if *&map[0][i].mark == b'.' {
            start = Some(Coordinate::new(0, i));
            break;
        }
    }
    let s = start.expect("!");
    let mut res: Vec<usize> = Vec::new();

    map[s.x][s.y].mark = VISITED;
    walk_part2(&mut res, &mut map, 0, &s);

    return Some(res.into_iter().max().unwrap() as i64);
}

fn walk_part2(
    end_steps: &mut Vec<usize>,
    map: &mut Vec<Vec<MapNode>>,
    steps: usize,
    pos: &Coordinate,
) {
    if steps < map[pos.x][pos.y].record {
        return;
    }
    map[pos.x][pos.y].record = steps;
    if pos.x == map.len() - 1 {
        end_steps.push(steps);
        // print_map(&map);
        return;
    }
    let neighbors = get_legal_neighbors_part2(pos, map);
    for neighbor in neighbors {
        map[neighbor.0.x][neighbor.0.y].mark = VISITED;
        walk_part2(end_steps, map, steps + neighbor.1, &neighbor.0);
        map[neighbor.0.x][neighbor.0.y].mark = PATH;
    }
}
fn get_legal_neighbors_part2(s: &Coordinate, map: &Vec<Vec<MapNode>>) -> Vec<(Coordinate, usize)> {
    let mut neighbors = Vec::new();
    if s.x > 0 && map[s.x - 1][s.y].mark != ROCK && map[s.x - 1][s.y].mark != VISITED {
        if map[s.x - 1][s.y].mark == b'^' {
            neighbors.push((Coordinate::new(s.x - 2, s.y), 2));
        } else {
            neighbors.push((Coordinate::new(s.x - 1, s.y), 1));
        }
    }
    if s.x + 1 < map.len() && map[s.x + 1][s.y].mark != ROCK && map[s.x + 1][s.y].mark != VISITED {
        if map[s.x + 1][s.y].mark == b'v' {
            neighbors.push((Coordinate::new(s.x + 2, s.y), 2));
        } else {
            neighbors.push((Coordinate::new(s.x + 1, s.y), 1));
        }
    }
    if s.y > 0 && map[s.x][s.y - 1].mark != ROCK && map[s.x][s.y - 1].mark != VISITED {
        if map[s.x][s.y - 1].mark == b'<' {
            neighbors.push((Coordinate::new(s.x, s.y - 2), 2));
        } else {
            neighbors.push((Coordinate::new(s.x, s.y - 1), 1));
        }
    }
    if s.y + 1 < map[0].len() && map[s.x][s.y + 1].mark != ROCK && map[s.x][s.y + 1].mark != VISITED
    {
        if map[s.x][s.y + 1].mark == b'>' {
            neighbors.push((Coordinate::new(s.x, s.y + 2), 2));
        } else {
            neighbors.push((Coordinate::new(s.x, s.y + 1), 1));
        }
    }
    neighbors
}
fn print_map(map: &Vec<Vec<u8>>) {
    map.iter().for_each(|v| {
        for c in v {
            print!("{:}", *c as char);
        }
        println!("");
    });
    println!("");
    println!("");
}
