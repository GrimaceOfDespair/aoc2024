use std::{
    fs::File, io::{prelude::*, BufReader}, path::Path
};

fn main() {
    println!("Hello, world!");
}

struct Dir {
    col: i32,
    row: i32,
}

#[derive(Copy, Clone)]
struct Guard {
    col: i32,
    row: i32,
    direction: usize,
}

#[derive(Copy, Clone, Debug)]
enum Item {
    Empty,
    Obstacle,
    NewObstacle,
    Covered,
}

struct CoveredItem {
    dirs: [bool; 4],
}

impl Into<char> for Item {
    fn into(self) -> char {
        self as u8 as char
    }
}

const UP: Dir = Dir { col: 0, row: -1 };
const RIGHT: Dir = Dir { col: 1, row: 0 };
const DOWN: Dir = Dir { col: 0, row: 1 };
const LEFT: Dir = Dir { col: -1, row: 0 };

const DIRECTIONS: [Dir; 4] = [UP, RIGHT, DOWN, LEFT];


fn _parse_file(filename: impl AsRef<Path>) -> (Vec<Vec<Item>>, Vec<Guard>) {

    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);
    let mut guards: Vec<Guard> = Vec::new();

    let grid = buf.lines().enumerate().map(|(row, line)|
        line.unwrap().chars().enumerate().map(|(col, pos)|
            match pos {
                '.' => Item::Empty,
                '#' => Item::Obstacle,
                '^' => { guards.push(Guard { col: col as i32, row: row as i32, direction: 0 }); Item::Empty },
                '>' => { guards.push(Guard { col: col as i32, row: row as i32, direction: 1 }); Item::Empty },
                'v' => { guards.push(Guard { col: col as i32, row: row as i32, direction: 2 }); Item::Empty },
                '<' => { guards.push(Guard { col: col as i32, row: row as i32, direction: 3 }); Item::Empty },
                _ => unreachable!()
            })
        .collect::<Vec<Item>>())
    .collect::<Vec<Vec<Item>>>();

    return (grid, guards);
}

fn move_guards(grid: &mut Vec<Vec<Item>>, guards: Vec<Guard>) -> Vec<Guard> {

    let moved_guards = guards.iter()
        .filter(|guard|
            guard.direction != 666)
        .map(|guard|
        {
            grid[guard.row as usize][guard.col as usize] = Item::Covered;

            let new_row = guard.row + DIRECTIONS[guard.direction].row;
            let new_col = guard.col + DIRECTIONS[guard.direction].col;
            if new_row < 0 || new_row >= grid.len() as i32 || new_col < 0 || new_col >= grid[0].len() as i32 {
                return Guard { row: -1, col: -1, direction: 666 }
            }

            return match grid[new_row as usize][new_col as usize] {
                Item::Empty | Item::Covered => Guard { row: new_row, col: new_col, direction: guard.direction },
                Item::Obstacle | Item::NewObstacle => Guard { row: guard.row, col: guard.col, direction: (guard.direction + 1) % 4 }
            }
        });

    return moved_guards
        .filter(|guard|
            guard.direction != 666)
        .collect::<Vec<Guard>>();
}

fn _move_guard(grid: &mut Vec<Vec<(Item, CoveredItem)>>, guard: &mut Guard) -> Option<bool> {

    grid[guard.row as usize][guard.col as usize].0 = Item::Covered;

    let new_row = guard.row + DIRECTIONS[guard.direction].row;
    let new_col = guard.col + DIRECTIONS[guard.direction].col;
    if new_row < 0 || new_row >= grid.len() as i32 || new_col < 0 || new_col >= grid[0].len() as i32 {
        return Some(false);
    }

    let new_row = new_row as usize;
    let new_col = new_col as usize;

    match grid[new_row][new_col].0 {
        Item::Empty | Item::Covered => {
            guard.col = new_col as i32;
            guard.row = new_row as i32;
            let direction = &mut grid[new_row][new_col].1.dirs[guard.direction];
            if !*direction {
                *direction = true;
            } else {
                return Some(true);
            }
        },
        Item::Obstacle | Item::NewObstacle => { 
            guard.direction = (guard.direction + 1) % 4
        },
    }

    return None;
}

fn _cover(grid: &mut Vec<Vec<(Item, CoveredItem)>>, guard: Guard) -> bool {
    let mut moving_guard = guard;
    loop  {
        let move_guard = _move_guard(grid, &mut moving_guard);

        if !move_guard.is_none() {
            return move_guard.unwrap();
        }
    }
}

fn _parse1(filename: impl AsRef<Path>) -> i32 {

    let (mut grid, mut guards) = _parse_file(filename);

    loop  {
        guards = move_guards(&mut grid, guards);
        if guards.len() == 0 { break; }
    }

    let covered = grid.iter()
        .flatten()
        .filter(|pos|
            matches!(pos, Item::Covered))
        .count();

    /* 
    for line in grid {
        println!("{}", line.iter().map(|pos| match pos {
            Item::Covered => 'X',
            Item::Empty => '.',
            Item::Obstacle => '#',
        }).collect::<String>());
    }
    */

    return covered as i32;
}

fn _create_variation(grid: &Vec<Vec<Item>>, obstacle: (i32, i32)) -> Vec<Vec<(Item, CoveredItem)>> {
    let variation = grid.iter().enumerate()
        .map(|(row, line)|
            line.iter()
                .enumerate()
                .map(|(col, item)|
                    (  
                        if obstacle.0 == row as i32 && obstacle.1 == col as i32 { Item::NewObstacle } else { *item },
                        CoveredItem { dirs: [false, false, false, false] })
                    )
                .collect::<Vec<(Item, CoveredItem)>>())
        .collect::<Vec<Vec<(Item, CoveredItem)>>>();

    return variation;
}

fn _parse2(filename: impl AsRef<Path>) -> i32 {
    let (grid, guards) = _parse_file(filename);

    let guard = guards[0];

    let mut directional_grid = grid
        .iter()
        .map(|line|
            line.iter().map(|pos|
                (*pos, CoveredItem { dirs: [false, false, false, false]}))
            .collect::<Vec<(Item, CoveredItem)>>())
        .collect::<Vec<Vec<(Item, CoveredItem)>>>();

    _cover(&mut directional_grid, guard);

    for line in &directional_grid {
        println!("{}", line.iter().map(|pos| match pos.0 {
            Item::Covered => 'X',
            Item::Empty => '.',
            Item::Obstacle => '#',
            Item::NewObstacle => 'O',
        }).collect::<String>());
    }

    let covered: Vec<(i32, i32)> = directional_grid.iter().enumerate()
        .map(|(row, line)|
            line.iter().enumerate().map(move |(col, cover)|
                if matches!(cover.0, Item::Covered) && !(row as i32 == guard.row && col as i32 == guard.col) {
                    Some((row as i32, col as i32))
                } else {
                    None
                }))
        .flatten()
        .filter_map(|pos| pos)
        .collect();

    println!("{}", covered.len());

    let mut i = 0;

    let obstructed = covered.iter()
        .filter_map(|cover| {
            let mut variation = _create_variation(&grid, *cover);

            let guard = guards[0];
            let guard_looping = _cover(&mut variation, guard);

            if guard_looping && false {
                i += 1;
                println!();
                println!("== {} =========================================================", i);
                println!();
                for line in &variation {
                    println!("{}", line.iter().map(|pos| match pos.0 {
                        Item::Covered => match pos.1.dirs {
                            [true, true, _, _] | [true, _, _, true] | [_, true, true, _] => '+',
                            [true, _, _, _] | [_, _, true, _] => '|',
                            [_, true, _, _] | [_, _, _, true] => '-',
                            _ => ' ',
                        },
                        Item::Empty => '.',
                        Item::Obstacle => '#',
                        Item::NewObstacle => 'O',
                    }).collect::<String>());
                }
            }
        
            if guard_looping { Some(true) } else { None }
        });

    return obstructed.count() as i32;
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