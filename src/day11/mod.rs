use std::cmp;

pub fn part01(input: &str) -> i64 {
    let mut cosmic: Vec<Vec<char>> = Vec::with_capacity(12);
    let mut no_galaxy_rows: Vec<usize> = Vec::new();
    let mut no_galaxy_cols: Vec<usize> = Vec::new();
    let mut points: Vec<(usize, usize)> = Vec::new();
    input.lines().enumerate().for_each(|(row, l)| {
        let mut no_galaxy = true;
        let mut line: Vec<char> = Vec::with_capacity(12);
        l.chars().enumerate().for_each(|(col, c)| {
            line.push(c);
            if c == '#' {
                points.push((row, col));
                no_galaxy = false;
            }
        });
        cosmic.push(line);
        if no_galaxy {
            no_galaxy_rows.push(row);
        }
    });

    for col in 0..cosmic[0].len() {
        let mut no_galaxy = true;
        for row in 0..cosmic.len() {
            if no_galaxy && cosmic[row][col] == '#' {
                no_galaxy = false;
            }
        }
        if no_galaxy {
            no_galaxy_cols.push(col);
        }
    }
    let mut res = 0;

    for i in 0..points.len() {
        for j in i..points.len() {
            let p1 = points.get(i).unwrap();
            let p2 = points.get(j).unwrap();

            let extra_moves = get_extra_moves(p1, p2, &no_galaxy_rows, &no_galaxy_cols) as i64;
            let normal_moves =
                (p1.0 as i64 - p2.0 as i64).abs() + (p1.1 as i64 - p2.1 as i64).abs();
            // println!(
            //     "{:?}-{:?} extra:{:?},total:{:?}",
            //     p1,
            //     p2,
            //     extra_moves,
            //     extra_moves + normal_moves
            // );
            res = res + extra_moves + normal_moves;
        }
    }

    return res;
}

fn get_extra_moves(
    p1: &(usize, usize),
    p2: &(usize, usize),
    no_galaxy_rows: &Vec<usize>,
    no_galaxy_cols: &Vec<usize>,
) -> usize {
    let mut extra_move: usize = 0;
    let small_row = cmp::min(p1.0, p2.0);
    let big_row = cmp::max(p1.0, p2.0);
    for no_galaxy_row in no_galaxy_rows {
        if no_galaxy_row > &small_row && no_galaxy_row < &big_row {
            extra_move = extra_move + 1;
        }
    }
    let small_col = cmp::min(p1.1, p2.1);
    let big_col = cmp::max(p1.1, p2.1);
    for no_galaxy_col in no_galaxy_cols {
        if no_galaxy_col > &small_col && no_galaxy_col < &big_col {
            extra_move = extra_move + 1;
        }
    }
    extra_move
}

pub fn part02(input: &str) -> i64 {
    return 0;
}
