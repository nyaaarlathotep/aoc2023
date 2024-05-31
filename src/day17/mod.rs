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
        steps: todo!(),
        i: todo!(),
        j: todo!(),
        dir: todo!(),
        limit: todo!(),
    });
    return Some(0);
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
    steps: usize,
    i: usize,
    j: usize,
    dir: Direction,
    limit: usize,
}

impl Grid {
    pub fn walk(&mut self, mut path: Path) {
        if path.i == self.height - 1 && path.j == self.width - 1 {
            self.points[path.i][path.j]
                .least_moves
                .entry((path.dir, path.limit))
                .and_modify(|num| {
                    if path.steps < *num {
                        *num = path.steps;
                    }
                })
                .or_insert(path.steps);
            return;
        }
        self.points[path.i][path.j]
            .least_moves
            .entry((path.dir, path.limit))
            .and_modify(|num| {
                if path.steps < *num {
                    *num = path.steps;
                } else {
                    return;
                }
            })
            .or_insert(path.steps);
        let mut next_path: Vec<Path> = Vec::new();
        match path.dir {
            Direction::North => {
                if path.limit > 0 {
                    let n = opt_north(path);
                    if let Some(north) = n {
                        next_path.push(north);
                    }
                }
                let e = opt_east(path, self.width);
                if let Some(east) = e {
                    next_path.push(east);
                }
                let w = opt_west(path);
                if let Some(west) = w {
                    next_path.push(west);
                }
            }
            Direction::South => {
                if path.limit > 0 {
                    let s = opt_south(path, self.height);
                    if let Some(south) = s {
                        next_path.push(south);
                    }
                }
                let e = opt_east(path, self.width);
                if let Some(east) = e {
                    next_path.push(east);
                }
                let w = opt_west(path);
                if let Some(west) = w {
                    next_path.push(west);
                }
            }
            Direction::East => {
                if path.limit > 0 {
                    let e = opt_east(path, self.width);
                    if let Some(east) = e {
                        next_path.push(east);
                    }
                }
                let n = opt_north(path);
                if let Some(north) = n {
                    next_path.push(north);
                }
                let s = opt_south(path, self.height);
                if let Some(south) = s {
                    next_path.push(south);
                }
            }
            Direction::West => {
                if path.limit > 0 {
                    let w = opt_west(path);
                    if let Some(west) = w {
                        next_path.push(west);
                    }
                }
                let n = opt_north(path);
                if let Some(north) = n {
                    next_path.push(north);
                }
                let s = opt_south(path, self.height);
                if let Some(south) = s {
                    next_path.push(south);
                }
            }
        }
        next_path.into_iter().for_each(|p| {
            self.walk(p);
        })
    }
}

fn opt_north(path: Path) -> Option<Path> {
    let mut s = None;
    if path.i > 0 {
        let mut north = path.clone();
        north.limit -= 1;
        north.steps = north.steps + 1;
        north.i = north.i - 1;
        s = Some(north);
    }
    s
}
fn opt_south(path: Path, height: usize) -> Option<Path> {
    let mut s = None;
    if path.i < height - 1 {
        let mut north = path.clone();
        north.limit -= 1;
        north.steps = north.steps + 1;
        north.i = north.i + 1;
        s = Some(north);
    }
    s
}
fn opt_west(path: Path) -> Option<Path> {
    let mut s = None;
    if path.i > 0 {
        let mut north = path.clone();
        north.limit -= 1;
        north.steps = north.steps + 1;
        north.i = north.j - 1;
        s = Some(north);
    }
    s
}
fn opt_east(path: Path, width: usize) -> Option<Path> {
    let mut s = None;
    if path.i < width - 1 {
        let mut north = path.clone();
        north.limit -= 1;
        north.steps = north.steps + 1;
        north.i = north.j + 1;
        s = Some(north);
    }
    s
}
