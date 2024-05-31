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
    let grid = Grid {
        width: points[0].len(),
        height: points.len(),
        points,
    };
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
                }else{
                    return;
                }
            })
            .or_insert(path.steps);

        
    }
}
