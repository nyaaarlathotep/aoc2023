use core::panic;
use std::collections::{hash_map::Entry, HashMap};

pub fn part01(input: &str) -> Option<i64> {
    let points: Vec<Vec<Point>> = input
        .lines()
        .map(|l| {
            let line: Vec<Point> = l
                .bytes()
                .map(|b| b as usize - 48)
                .map(|loss| Point {
                    loss,
                    least_moves: HashMap::new(),
                })
                .collect();
            return line;
        })
        .collect::<Vec<Vec<Point>>>();
    let mut grid = Grid {
        width: points[0].len(),
        height: points.len(),
        points,
    };
    grid.walk(Path {
        total_loss: 0,
        i: 0,
        j: 0,
        dir: Direction::East,
        limit: 3,
    });
    let min_move = grid.points[grid.height - 1][grid.width - 1]
        .least_moves
        .iter()
        .map(|m| {
            return m.1;
        })
        .min()
        .unwrap();
    return Some(*min_move as i64);
}

pub fn part02(input: &str) -> Option<i64> {
    return Some(0);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Grid {
    points: Vec<Vec<Point>>,
    width: usize,
    height: usize,
}
struct Point {
    loss: usize,
    least_moves: HashMap<(Direction, usize), usize>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Path {
    total_loss: usize,
    i: usize,
    j: usize,
    dir: Direction,
    limit: usize,
}

impl Grid {
    pub fn walk(&mut self, path: Path) {
        if path.i == self.height - 1 && path.j == self.width - 1 {
            self.points[path.i][path.j]
                .least_moves
                .entry((path.dir, path.limit))
                .and_modify(|num| {
                    if path.total_loss < *num {
                        *num = path.total_loss;
                    }
                })
                .or_insert(path.total_loss);
            return;
        }
        self.points[path.i][path.j]
            .least_moves
            .entry((path.dir, path.limit))
            .and_modify(|num| {
                if path.total_loss < *num {
                    *num = path.total_loss;
                } else {
                    return;
                }
            })
            .or_insert(path.total_loss);
        let mut next_paths: Vec<Direction> = Vec::new();
        match path.dir {
            Direction::North => {
                if path.limit > 0 {
                    let n = opt_north(path);
                    if let Some(north) = n {
                        next_paths.push(north);
                    }
                }
                let e = opt_east(path, self.width);
                if let Some(east) = e {
                    next_paths.push(east);
                }
                let w = opt_west(path);
                if let Some(west) = w {
                    next_paths.push(west);
                }
            }
            Direction::South => {
                if path.limit > 0 {
                    let s = opt_south(path, self.height);
                    if let Some(south) = s {
                        next_paths.push(south);
                    }
                }
                let e = opt_east(path, self.width);
                if let Some(east) = e {
                    next_paths.push(east);
                }
                let w = opt_west(path);
                if let Some(west) = w {
                    next_paths.push(west);
                }
            }
            Direction::East => {
                if path.limit > 0 {
                    let e = opt_east(path, self.width);
                    if let Some(east) = e {
                        next_paths.push(east);
                    }
                }
                let n = opt_north(path);
                if let Some(north) = n {
                    next_paths.push(north);
                }
                let s = opt_south(path, self.height);
                if let Some(south) = s {
                    next_paths.push(south);
                }
            }
            Direction::West => {
                if path.limit > 0 {
                    let w = opt_west(path);
                    if let Some(west) = w {
                        next_paths.push(west);
                    }
                }
                let n = opt_north(path);
                if let Some(north) = n {
                    next_paths.push(north);
                }
                let s = opt_south(path, self.height);
                if let Some(south) = s {
                    next_paths.push(south);
                }
            }
        }
        next_paths.into_iter().for_each(|p| {
            let mut next_path = path.clone();
            next_path.dir = p;
            next_path.total_loss += self.points[path.i][path.j].loss;
            if p != path.dir {
                next_path.limit = 4;
            }
            if next_path.limit == 0 {
                panic!("turn err!");
            }
            next_path.limit -= 1;

            self.walk(next_path);
        })
    }
}

fn opt_north(path: Path) -> Option<Direction> {
    if path.i > 0 {
        return Some(Direction::North);
    }
    None
}
fn opt_south(path: Path, height: usize) -> Option<Direction> {
    if path.i < height - 1 {
        return Some(Direction::South);
    }
    None
}
fn opt_west(path: Path) -> Option<Direction> {
    if path.j > 0 {
        return Some(Direction::West);
    }
    None
}
fn opt_east(path: Path, width: usize) -> Option<Direction> {
    if path.i < width - 1 {
        return Some(Direction::East);
    }
    None
}
