fn main() {
    println!("Hello, world!");
}

use std::{
    path::Path,
    fs::File,
    io::{prelude::*, BufReader},
};

const _DIRECTIONS: [[i32; 2]; 8] = [
    [-1, -1], [-1, 0], [-1, 1],
    [ 0, -1],          [ 0, 1],
    [ 1, -1], [ 1, 0], [ 1, 1]
];

const _X_MAS_DIRECTIONS: [[i32; 2]; 4] = [[-1, -1], [-1, 1], [1, 1], [1, -1]];
const _X_MAS: [char; 4] = ['M', 'S', 'S', 'M'];

fn _filter_letter(grid: &Vec<Vec<(usize, usize, char)>>, row: usize, col: usize, c: char, d: [i32; 2]) -> bool {

    let new_row = row as i32 + d[0];
    let new_col = col as i32 + d[1];
    let grid_height = grid.len() as i32;
    let grid_width = grid[0].len() as i32;

    if new_row < 0 || new_row > grid_height - 1 { return false; }
    if new_col < 0 || new_col > grid_width - 1 { return false; }

    let result = grid[new_row as usize][new_col as usize].2 == c;

    return result;
}

fn _parse_file(filename: impl AsRef<Path>) -> Vec<Vec<(usize, usize, char)>> {
    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);

    let grid = buf.lines().enumerate().map(|(row, line)|
        line.unwrap().char_indices().map(|(col, c)| (row, col, c)).collect())
        .collect::<Vec<Vec<(usize, usize, char)>>>();

    return grid;
}

fn _parse1(filename: impl AsRef<Path>) -> usize {

    let grid = _parse_file(filename);

    let all_x = grid.iter()
        .flatten()
        .filter(|x|
            x.2 == 'X')
        .map(|x|
            _DIRECTIONS.map(|d| (x.0, x.1, x.2, d)))
        .flatten();

    let xmas = all_x
        .filter(|x|
            _filter_letter(&grid, x.0, x.1, 'M', x.3))
        .filter(|m|
            _filter_letter(&grid, m.0, m.1, 'A', [m.3[0] * 2, m.3[1] * 2]))
        .filter(|a|
            _filter_letter(&grid, a.0, a.1, 'S', [a.3[0] * 3, a.3[1] * 3]))
        .collect::<Vec<(usize, usize, char, [i32; 2])>>();

    return xmas.len();
}

fn _parse2(filename: impl AsRef<Path>) -> usize {

    let grid = _parse_file(filename);

    let all_a = grid.iter()
        .flatten()
        .filter(|x|
            x.2 == 'A')
        .map(|x|
            (0..4).map(|rot| (x.0, x.1, x.2, rot)))
        .flatten();
    
    let xmas = all_a
        .filter(|a|
            _X_MAS_DIRECTIONS.iter().enumerate().all(|(i, d)|
                _filter_letter(&grid, a.0, a.1, _X_MAS[(a.3 + i) % 4], *d)))
        .collect::<Vec<(usize, usize, char, usize)>>();

    return xmas.len();
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