use std::{
    collections::HashSet, fs::File, io::{prelude::*, BufReader}, path::Path, usize
};
use regex::Regex;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coords {
    col: i64,
    row: i64,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: Coords,
    mov: Coords,
}

struct Bathroom {
    robots: Vec<Robot>,
    cols: i64,
    rows: i64,
}

fn _parse_file(filename: impl AsRef<Path>, rows: i64, cols: i64) -> Bathroom {

    let regex = Regex::new(r"p\=(-?\d+),(-?\d+) v\=(-?\d+),(-?\d+)").unwrap();

    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);

    let robots = buf.lines()
        .map(|line|
            {
                let text = line.unwrap();
                let captures = regex.captures(&text).unwrap();
                let col = captures.get(1).unwrap().as_str().parse::<i64>().unwrap() as i64;
                let row = captures.get(2).unwrap().as_str().parse::<i64>().unwrap() as i64;
                let col_mov = captures.get(3).unwrap().as_str().parse::<i64>().unwrap() as i64;
                let row_mov = captures.get(4).unwrap().as_str().parse::<i64>().unwrap() as i64;

                Robot {
                    pos: Coords {
                        col,
                        row,
                    },
                    mov: Coords {
                        col: col_mov,
                        row: row_mov,
                    }
                }
            })
            .collect::<Vec<Robot>>();

        Bathroom {
            robots,
            cols,
            rows,

        }
}

fn _print_bathroom(bathroom: &Bathroom) {
    _print_robots(&bathroom.robots, bathroom.rows, bathroom.cols);
}

fn _print_robots(robots: &Vec<Robot>, rows: i64, cols: i64) {
    println!();

    for row in 0..rows {
        for col in 0..cols {
            let robot_count =  robots.iter()
                .filter(|robot|
                    robot.pos.row == row &&
                    robot.pos.col == col)
                .count();

            let middle_row = row == ((rows - 1) / 2);
            let middle_col = col == ((cols - 1) / 2);

            if robot_count == 0 {
                if middle_col && middle_row {
                    print!("+");
                } else if middle_col {
                    print!("|");
                } else if middle_row {
                    print!("-");
                } else {
                    print!(".");
                }
            } else {
                print!("{robot_count}");
            }
        }
        println!();
    }
    println!();
}

fn move_robots_continuous(bathroom: &Bathroom, moves: i64) -> Vec<Robot> {
    bathroom.robots.iter()
        .map(|robot| {
            let new_col = (((robot.pos.col + (robot.mov.col * moves)) % bathroom.cols) + bathroom.cols) % bathroom.cols;
            let new_row = (((robot.pos.row + (robot.mov.row * moves)) % bathroom.rows) + bathroom.rows) % bathroom.rows;

            if new_col < 0 || new_row < 0 {
                unreachable!();
            }

            Robot {
                pos: Coords {
                    col: new_col,
                    row: new_row,
                },
                mov: Coords {
                    col: robot.mov.col,
                    row: robot.mov.row,
                }
            }
        })
        .collect::<Vec<Robot>>()
}

fn move_robots_unique(bathroom: &Bathroom) -> usize {

    let mut moves = 0;
    let mut done = false;
    let mut robots: Vec<Robot> = bathroom.robots.clone();
    let mut unique: HashSet<Coords> = HashSet::new();

    while !done {

        robots = robots.iter().map(|robot| {

            let col = (robot.pos.col + robot.mov.col + bathroom.cols) % bathroom.cols;
            let row = (robot.pos.row + robot.mov.row + bathroom.rows) % bathroom.rows;

            let pos = Coords {
                col,
                row,
            };

            unique.insert(pos);

            Robot {
                pos,
                mov: robot.mov.clone()
            }
        })
        .collect::<Vec<Robot>>();

        done = unique.len() == robots.len();

        unique.clear();

        moves += 1;
    }
    
    _print_robots(&robots, bathroom.rows, bathroom.cols);

    moves

}

fn move_robots_discrete(bathroom: &Bathroom, moves: i64) -> Vec<Robot> {

    let robots = bathroom.robots.iter()
        .map(|r|
            *r)
        .collect::<Vec<Robot>>();

    (0..moves).fold(robots, |acc, _|
        {
            let new_positions: Vec<Robot> = acc.iter().map(|robot| {
                let mut new_col = robot.pos.col + robot.mov.col;
                let mut new_row = robot.pos.row + robot.mov.row;

                if new_col < 0 {
                    new_col = new_col + bathroom.cols;
                } else if new_col >= bathroom.cols{
                    new_col = new_col - bathroom.cols;
                }

                if new_row < 0 {
                    new_row = new_row + bathroom.rows;
                } else if new_row >= bathroom.rows {
                    new_row = new_row - bathroom.rows;
                }
                //println!("[{} {}] + [{} {}] = [{}, {}]", robot.pos.x, robot.pos.y, robot.mov.x, robot.mov.y, new_x, new_y);

                Robot {
                    pos: Coords {
                        col: new_col,
                        row: new_row,
                    },
                    mov: Coords {
                        col: robot.mov.col,
                        row: robot.mov.row,
                    }
                }
            })
            .collect::<Vec<Robot>>();

            //_print_robots(&new_positions, bathroom.rows, bathroom.cols);

            new_positions
        })
}


fn _collect_quadrants(bathroom: &Bathroom) -> [i64; 4] {
    let middle_col = (bathroom.cols - 1) / 2;
    let middle_row = (bathroom.rows - 1) / 2;

    let quadrants = bathroom.robots.iter()
        .filter_map(|robot| {
            if robot.pos.col == middle_col || robot.pos.row == middle_row {
                //println!("skipping [{}, {}]", robot.pos.col, robot.pos.row);
                None
            } else {
                Some(if robot.pos.col < middle_col && robot.pos.row < middle_row {
                    0usize
                } else if robot.pos.col > middle_col && robot.pos.row < middle_row {
                    1usize
                } else if robot.pos.col < middle_col && robot.pos.row > middle_row {
                    2usize
                } else {
                    3usize
                })
            }
        })
        .collect::<Vec<usize>>();

    let mut unique_quadrants: [i64; 4] = [0, 0, 0, 0];

    for quadrant in quadrants {
        unique_quadrants[quadrant] = unique_quadrants[quadrant] + 1;
    }

    unique_quadrants
}

fn _parse1() -> usize {
    let moves = 100;
    let bathroom = _parse_file("src/sample", 7, 11);
    //let bathroom = _parse_file("src/input", 103, 101);

    println!("Planning {} moves on a bathroom of {} tiles wide and {} tiles tall",
        moves, bathroom.cols, bathroom.rows);

    _print_bathroom(&bathroom);

    let moved_bathroom = Bathroom {
        robots: move_robots_continuous(&bathroom, moves),
        ..bathroom
    };

    _print_bathroom(&moved_bathroom);

    let quadrants = _collect_quadrants(&moved_bathroom);

    println!("{:?}", quadrants);

    (quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]) as usize

}

fn _parse2() -> usize {
    let bathroom = _parse_file("src/sample", 7, 11);
    let bathroom = _parse_file("src/input", 103, 101);

    let moves = move_robots_unique(&bathroom);

    //_print_bathroom(&moved_bathroom);

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output() {
        let result1 = _parse1();
        println!("done 1: {result1}");
        let result1 = _parse2();
        println!("done 2: {result1}");
        assert!(false);
    }
}