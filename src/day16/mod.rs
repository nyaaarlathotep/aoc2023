use std::{
    fmt::{Display, Write},
    str::FromStr,
};

pub fn part01(input: &str) -> Option<i64> {
    let light = Light {
        dir: Direction::East,
        i: 0,
        j: 0,
    };
    Some(iin(input, light).unwrap() as i64)
}

fn iin(input: &str, light: Light) -> Option<usize> {
    let mut grid = Grid::from_str(input).unwrap();
    grid.incidence(light);
    let mut total = 0;
    for i in 0..grid.height {
        for j in 0..grid.width {
            if grid.tiles[i][j].lights.len() > 0 {
                total = total + 1;
            }
        }
    }
    return Some(total);
}

pub fn part02(input: &str) -> Option<i64> {
    let mut max: usize = 0;
    for i in 0..109 {
        let light = Light {
            dir: Direction::East,
            i: i,
            j: 0,
        };
        let max_this = iin(input, light).unwrap();
        if max_this > max {
            max = max_this;
        }
    }
    for i in 0..109 {
        let light = Light {
            dir: Direction::West,
            i: i,
            j: 109,
        };
        let max_this = iin(input, light).unwrap();
        if max_this > max {
            max = max_this;
        }
    }
    for j in 0..109 {
        let light = Light {
            dir: Direction::South,
            i: 0,
            j: j,
        };
        let max_this = iin(input, light).unwrap();
        if max_this > max {
            max = max_this;
        }
    }
    for j in 0..109 {
        let light = Light {
            dir: Direction::West,
            i: 109,
            j: j,
        };
        let max_this = iin(input, light).unwrap();
        if max_this > max {
            max = max_this;
        }
    }
    return Some(max as i64);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Light {
    dir: Direction,
    i: usize,
    j: usize,
}
struct Grid {
    tiles: Vec<Vec<Pos>>,
    width: usize,
    height: usize,
}
struct Pos {
    tile: Tile,
    lights: Vec<Direction>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    RightMirror,
    LeftMirror,
    VerticalSplitter,
    HorizontalSplitter,
}
#[derive(Debug)]
struct ParseGridError;
impl FromStr for Grid {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Vec<Pos>> = s
            .lines()
            .map(|l| {
                let tile_line = l
                    .as_bytes()
                    .into_iter()
                    .map(|b| {
                        let a: Result<Pos, ParseGridError> = Ok(match b {
                            b'.' => Pos {
                                tile: Tile::Empty,
                                lights: Vec::with_capacity(4),
                            },
                            b'\\' => Pos {
                                tile: Tile::LeftMirror,
                                lights: Vec::with_capacity(4),
                            },
                            b'/' => Pos {
                                tile: Tile::RightMirror,
                                lights: Vec::with_capacity(4),
                            },
                            b'|' => Pos {
                                tile: Tile::VerticalSplitter,
                                lights: Vec::with_capacity(4),
                            },
                            b'-' => Pos {
                                tile: Tile::HorizontalSplitter,
                                lights: Vec::with_capacity(4),
                            },
                            _ => return Err(ParseGridError),
                        });
                        return a;
                    })
                    .collect::<Result<Vec<Pos>, ParseGridError>>()?;
                return Ok(tile_line);
            })
            .collect::<Result<Vec<Vec<Pos>>, ParseGridError>>()?;
        Ok(Self {
            width: tiles[0].len(),
            height: tiles.len(),
            tiles,
        })
    }
}

impl Grid {
    pub fn incidence(&mut self, light: Light) {
        // println!("{}", self);
        if light.i >= self.width || light.j >= self.height {
            return;
        }
        let poses = &mut self.tiles[light.i][light.j];
        if poses.lights.contains(&light.dir) {
            return;
        }
        poses.lights.push(light.dir);
        if poses.tile == Tile::Empty {
            if let Some(next_light) = next_pos(light) {
                self.incidence(next_light);
            };
            return;
        }
        if poses.tile == Tile::HorizontalSplitter {
            if light.dir == Direction::West || light.dir == Direction::East {
                if let Some(next_light) = next_pos(light) {
                    self.incidence(next_light);
                };
                return;
            }
            if light.j > 0 {
                let left = Light {
                    dir: Direction::West,
                    i: light.i,
                    j: light.j - 1,
                };
                self.incidence(left);
            }

            let right = Light {
                dir: Direction::East,
                i: light.i,
                j: light.j + 1,
            };
            self.incidence(right);
            return;
        }
        if poses.tile == Tile::VerticalSplitter {
            if light.dir == Direction::North || light.dir == Direction::South {
                if let Some(next_light) = next_pos(light) {
                    self.incidence(next_light);
                };
                return;
            }
            if light.i > 0 {
                let up = Light {
                    dir: Direction::North,
                    i: light.i - 1,
                    j: light.j,
                };
                self.incidence(up);
            }
            let right = Light {
                dir: Direction::South,
                i: light.i + 1,
                j: light.j,
            };
            self.incidence(right);
            return;
        }
        if let Some(next_light) = next_mirrir_pos(light, poses.tile) {
            self.incidence(next_light);
        };
    }
}

fn next_pos(mut light: Light) -> Option<Light> {
    match light.dir {
        Direction::North => {
            if light.i == 0 {
                return None;
            }
            light.i = light.i - 1
        }
        Direction::South => light.i = light.i + 1,
        Direction::East => light.j = light.j + 1,
        Direction::West => {
            if light.j == 0 {
                return None;
            }
            light.j = light.j - 1
        }
    }
    return Some(light);
}
fn next_mirrir_pos(mut light: Light, tile: Tile) -> Option<Light> {
    match tile {
        Tile::LeftMirror => match light.dir {
            Direction::North => {
                if light.j == 0 {
                    return None;
                }
                light.dir = Direction::West;
                light.j = light.j - 1;
            }
            Direction::South => {
                light.dir = Direction::East;
                light.j = light.j + 1;
            }
            Direction::East => {
                light.dir = Direction::South;
                light.i = light.i + 1;
            }
            Direction::West => {
                if light.i == 0 {
                    return None;
                }
                light.dir = Direction::North;
                light.i = light.i - 1
            }
        },
        Tile::RightMirror => match light.dir {
            Direction::North => {
                light.dir = Direction::East;
                light.j = light.j + 1;
            }
            Direction::South => {
                if light.j == 0 {
                    return None;
                }
                light.dir = Direction::West;
                light.j = light.j - 1;
            }
            Direction::East => {
                if light.i == 0 {
                    return None;
                }
                light.dir = Direction::North;
                light.i = light.i - 1;
            }
            Direction::West => {
                light.dir = Direction::South;
                light.i = light.i + 1
            }
        },
        _ => {
            panic!("{:?}", light.dir)
        }
    }

    return Some(light);
}
impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tile)?;
        Ok(())
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => f.write_char('.'),
            Tile::RightMirror => f.write_char('/'),
            Tile::LeftMirror => f.write_char('\\'),
            Tile::VerticalSplitter => f.write_char('|'),
            Tile::HorizontalSplitter => f.write_char('-'),
        }
    }
}
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.tiles[i][j].lights.len() > 0 {
                    write!(f, "# ")?;
                } else {
                    write!(f, "{} ", self.tiles[i][j])?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
