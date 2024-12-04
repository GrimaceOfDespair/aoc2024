use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::{Path},
    collections::HashMap,
};

fn parse_file(filename: impl AsRef<Path>) -> Vec<(i32, i32)> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    return buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .map(|line| {
            let elements = line.split_whitespace();
            let numbers: Vec<i32> = elements
                .map(|e| e.parse().unwrap())
                .collect();

            return (numbers[0], numbers[1]);
        })
        .collect();
}

fn _parse1(filename: impl AsRef<Path>) -> i32 {

    let numbers = parse_file(filename);
    let mut left: Vec<i32> = numbers.iter().map(|x| x.0).collect();
    let mut right: Vec<i32> = numbers.iter().map(|x| x.1).collect();
    left.sort();
    right.sort();

    let distances = left.iter().zip(right.iter())
        .map(|(l, r)| (r - l).abs())
        .sum::<i32>();

    return distances;
}

fn parse2(filename: impl AsRef<Path>) -> i32 {

    let numbers = parse_file(filename);
    let mut occurrences: HashMap<i32, i32> = HashMap::new();
    for number in &numbers {
        *occurrences.entry(number.1).or_insert(0) += 1;
    }

    let score = numbers.iter()
        .map(|l|
            *occurrences.entry(l.0).or_insert(0) * l.0)
        .sum::<i32>();

    return score
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output() {
        let result = parse2("src/input");
        println!("done: {}", result);
        assert!(false);
    }
}