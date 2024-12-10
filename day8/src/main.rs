use std::{
    cmp::max, collections::{HashMap, HashSet}, fs::File, io::{prelude::*, BufReader}, path::Path
};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    col: i64,
    row: i64,
}

#[derive(Debug, Clone, Copy)]
struct Antenna {
    c: char,
    col: usize,
    row: usize,
}

struct Grid {
    antennas: HashMap<char, Vec<Antenna>>,
    rows: usize,
    cols: usize,
}

fn _parse_file(filename: impl AsRef<Path>) -> Grid {

    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);
    let lines = buf.lines();
    let mut rows: usize = 0;
    let mut cols: usize = 0;

    let items = lines.enumerate().map(|(row, line)|
        line.unwrap().chars().enumerate().filter_map(|(col, pos)|
            {
                rows = max(rows, row + 1);
                cols = max(cols, col + 1);

                match pos {
                    '.' | '#' => None,
                    c => Some((c, Antenna {
                        c,
                        col,
                        row,
                    }),)
            }
            })
            .collect::<Vec<(char, Antenna)>>())
        .flatten()
        .collect::<Vec<(char, Antenna)>>();

    let mut antennas: HashMap<char, Vec<Antenna>> = HashMap::new();
    for (c, antenna) in items {
        antennas.entry(c).or_insert(Vec::new()).push(antenna);
    }

    println!("{} x {}", cols, rows);

    Grid { antennas, rows, cols }
}

fn _pairs(antennas: &Vec<Antenna>) -> Vec<(Antenna, Antenna)> {
    let pairs = antennas.iter().skip(1)
        .map(|antenna|
            (antennas[0], *antenna));

    if antennas.len() < 2 {
        pairs.collect::<Vec<(Antenna, Antenna)>>()
    } else {
        let tail = _pairs(&antennas.iter().skip(1).map(|a| *a).collect::<Vec<Antenna>>());
        pairs.chain(tail).collect::<Vec<(Antenna, Antenna)>>()
    }
}

fn _parse1(filename: impl AsRef<Path>) -> i64 {
    let Grid { antennas, rows, cols } = _parse_file(filename);

    let signals = antennas.iter().map(|group|
        _pairs(group.1).iter().map(|pair|
            {
                let c = pair.0.c;
                let row_offset = pair.1.row as i64 - pair.0.row as i64;
                let col_offset = pair.1.col as i64 - pair.0.col as i64;
                let signal1_row = pair.0.row as i64 - row_offset as i64;
                let signal1_col = pair.0.col as i64 - col_offset as i64;
                let signal2_row = pair.1.row as i64 + row_offset as i64;
                let signal2_col = pair.1.col as i64 + col_offset as i64;

                //println!("[col {: >2}, row {: >2}] x [col {: >2}, row {: >2}] => [col {: >2}, row {: >2}] x [col {: >2}, row {: >2}]",
                //    pair.0.col, pair.0.row, pair.1.col, pair.1.row, signal1_row, signal1_col, signal2_row, signal2_col);

                vec![
                    Pos {
                        row: signal1_row,
                        col: signal1_col,
                    },
                    Pos {
                        row: signal2_row,
                        col: signal2_col,
                    }
                ]
            })
            .flatten()
            .collect::<Vec<Pos>>())
        .flatten()
        .collect::<Vec<Pos>>();

    
    let mut filtered_signals =    
        signals.iter().filter(|signal|
            signal.col >= 0 && signal.col < cols as i64 &&
            signal.row >= 0 && signal.row < rows as i64)
        .map(|x| *x)
        .collect::<Vec<Pos>>();

    // for row in 0..rows {
    //     print!("{: >2}: ", row);
    //     for col in 0..cols {
    //         let c = (&filtered_signals).iter().find(|signal|
    //             signal.row == row as i64 &&
    //             signal.col == col as i64);

    //         let c = if c.is_some()  {
    //             '#'
    //         } else {
    //             '.'
    //         };
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    let mut uniques = HashSet::new();
    filtered_signals.retain(|e| uniques.insert(*e));
        
    uniques.len() as i64
}

fn _parse2(filename: impl AsRef<Path>) -> i64 {
    let Grid { antennas, rows, cols } = _parse_file(filename);

    let signals = antennas.iter().map(|group|
        _pairs(group.1).iter().map(|pair|
            {
                let c = pair.0.c;
                let row_offset = pair.1.row as i64 - pair.0.row as i64;
                let col_offset = pair.1.col as i64 - pair.0.col as i64;
                let mut signal1_row = pair.0.row as i64 - row_offset as i64;
                let mut signal1_col = pair.0.col as i64 - col_offset as i64;
                let mut signal2_row = pair.1.row as i64 + row_offset as i64;
                let mut signal2_col = pair.1.col as i64 + col_offset as i64;

                let mut signals: Vec<Pos> = Vec::new();
                while signal1_row >= 0 && signal1_col >= 0 {
                    signals.push(Pos {
                        row: signal1_row,
                        col: signal1_col,
                    });

                    signal1_row -= row_offset;
                    signal1_col -= col_offset;
                }

                while signal2_row < rows as i64 && signal2_col < cols as i64 {
                    signals.push(Pos {
                        row: signal2_row,
                        col: signal2_col,
                    });

                    signal2_row += row_offset;
                    signal2_col += col_offset;
                }

                // println!("{} x {}", rows, cols);

                // for row in 0..rows {
                //     print!("{: >2}: ", row);
                //     for col in 0..cols {
                //         let c = (&signals).iter().find(|signal|
                //             signal.row == row as i64 &&
                //             signal.col == col as i64);
            
                //         let c = if c.is_some()  {
                //             '#'
                //         } else {
                //             if pair.0.row == row && pair.0.col == col || pair.1.row == row && pair.1.col == col {
                //                 pair.0.c
                //             } else {
                //                 '.'
                //             }
                //         };
                //         print!("{}", c);
                //     }
                //     println!();
                // }
                        
                signals

            })
            .flatten()
            .collect::<Vec<Pos>>())
        .flatten()
        .collect::<Vec<Pos>>();

    
    let all_antennas = antennas.iter()
        .map(|antenna| antenna.1)
        .flatten()
        .collect::<Vec<&Antenna>>();

    let mut count = 0i64;

    for row in 0..rows {
        print!("{: >2}: ", row);
        for col in 0..cols {
            let c = (&signals).iter().find(|signal|
                signal.row == row as i64 &&
                signal.col == col as i64);

            let c = if c.is_some()  {
                count += 1;
                '#'
            } else {
                let antenna = all_antennas.iter().find(|a| a.row == row && a.col == col);
                if antenna.is_some() {
                    count += 1;
                    antenna.unwrap().c
                } else {
                    '.'
                }
            };
            print!("{}", c);
        }
        println!();
    }

    count
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