use std::{
    collections::{HashMap, HashSet}, fs::File, io::{prelude::*, BufReader}, path::Path
};

fn main() {
    println!("Hello, world!");
}

fn _parse_file(filename: impl AsRef<Path>) -> Vec<Vec<i64>> {

    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);
    let files = buf.lines().map(|line|
        line.unwrap().chars()
            .map(|c|
                if let Some(i) = c.to_digit(10) { i as i64 } else { -1 })
            .collect::<Vec<i64>>())
        .collect::<Vec<Vec<i64>>>();

    files
}

fn _parse1(filename: impl AsRef<Path>) -> usize {
    let grid = _parse_file(filename);

    let init = grid.iter()
        .enumerate()
        .map(|(row, line)|
            line.iter().enumerate()
                .map(|(col, x)| (row as i64, col as i64))
                .collect::<Vec<(i64, i64)>>())
        .flatten()
        .filter(|pos| grid[pos.0 as usize][pos.1 as usize] == 0)
        .collect::<Vec<(i64, i64)>>();

    let rows = grid.len() as i64;
    let cols = grid[0].len() as i64;

    let result = init.iter().map(|square0| {
        let result = (1i64..10i64)
            .fold(vec!(*square0), |acc, step|
                acc.iter().map(|(row, col)|
                    vec!((-1i64, 0i64), (0i64, 1i64), (1i64, 0i64), (0i64, -1i64)).iter()
                        .filter_map(|(jump_row, jump_col)|
                            {
                                let next_row = row + jump_row;
                                let next_col = col + jump_col;
                                
                                if next_row < 0 || next_row >= rows ||
                                    next_col < 0 || next_col >= cols {
                                    None
                                } else {
                                    let pos_next = grid[next_row as usize][next_col as usize];
                                    if pos_next as i64 == step {
                                        //println!("{} [{}, {}] => [{}, {}]", step, row, col, next_row, next_col);
                                        Some((next_row, next_col))
                                    } else {
                                        None
                                    }
                                }
                            })
                        .collect::<Vec<(i64, i64)>>())
                    .flatten()
                    .collect::<Vec<(i64, i64)>>()
                );

        let mut unique_trails: HashSet<(i64, i64)> = HashSet::new();
        for destination in result {
            unique_trails.insert(destination);
        }

        unique_trails.len() as usize
    })
    .collect::<Vec<usize>>();
    
    result.iter().sum()
}

fn _parse2(filename: impl AsRef<Path>) -> usize {
    let grid = _parse_file(filename);

    let init = grid.iter()
        .enumerate()
        .map(|(row, line)|
            line.iter().enumerate()
                .map(|(col, x)| (row as i64, col as i64))
                .collect::<Vec<(i64, i64)>>())
        .flatten()
        .filter(|pos| grid[pos.0 as usize][pos.1 as usize] == 0)
        .collect::<Vec<(i64, i64)>>();

    let rows = grid.len() as i64;
    let cols = grid[0].len() as i64;

    let result = init.iter().map(|square0| {
        let result = (1i64..10i64)
            .fold(vec!(*square0), |acc, step|
                acc.iter().map(|(row, col)|
                    vec!((-1i64, 0i64), (0i64, 1i64), (1i64, 0i64), (0i64, -1i64)).iter()
                        .filter_map(|(jump_row, jump_col)|
                            {
                                let next_row = row + jump_row;
                                let next_col = col + jump_col;
                                
                                if next_row < 0 || next_row >= rows ||
                                    next_col < 0 || next_col >= cols {
                                    None
                                } else {
                                    let pos_next = grid[next_row as usize][next_col as usize];
                                    if pos_next as i64 == step {
                                        //println!("{} [{}, {}] => [{}, {}]", step, row, col, next_row, next_col);
                                        Some((next_row, next_col))
                                    } else {
                                        None
                                    }
                                }
                            })
                        .collect::<Vec<(i64, i64)>>())
                    .flatten()
                    .collect::<Vec<(i64, i64)>>()
                );

        result.len() as usize
    })
    .collect::<Vec<usize>>();
    
    result.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output() {
        let result1 = _parse1("src/input");
        println!("done 1: {result1}");
        let result2 = _parse2("src/input");
        println!("done 2: {result2}");
        assert!(false);
    }
}