fn main() {
    println!("Hello, world!");
}

use std::{
    path::Path,
    fs::File,
    io::{prelude::*, BufReader},
};
use regex::{Regex};

fn _parse_file(filename: impl AsRef<Path>) -> Vec<[i32; 2]> {
    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);

    return buf.lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
            let operations = re.captures_iter(&line).map(|mul|
                (mul.extract::<2>().1)
                    .map(|op| op.parse::<i32>().unwrap()))
                .collect::<Vec<[i32; 2]>>();

            return operations;
        })
        .flatten()
        .collect::<Vec<[i32; 2]>>();
}

fn _parse_file2b(filename: impl AsRef<Path>) -> Vec<[i32; 2]> {

    let file = File::open(filename).unwrap();
    let mut buf = BufReader::new(file);
    let mut input = String::new();
    buf.read_to_string(&mut input).expect("Cannot read file");

    let filtered_input = input.split("do()")
        .map(|operations| {
            let operation =  operations.split_once("don't()");

            if operation == None {
                return operations;
            } else {
                return operation.unwrap().0;
            }
        })
        .collect::<Vec<&str>>()
        .join("");

    let multiplications = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let operations = multiplications.captures_iter(&filtered_input).map(|mul|
        (mul.extract::<2>().1)
            .map(|op| op.parse::<i32>().unwrap()))
        .collect::<Vec<[i32; 2]>>();

    return operations;
}


fn _parse_file2(filename: impl AsRef<Path>) -> Vec<[i32; 2]> {
    let file = File::open(filename).unwrap();
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    buf.read_to_string(&mut line).expect("Cannot read file");

    let filter: Regex = Regex::new(r"don\'t\(\).*?do\(\)").unwrap();
    let filtered_line = filter.replace_all(&line, "");

    let multiplications = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let operations = multiplications.captures_iter(&filtered_line).map(|mul|
        (mul.extract::<2>().1)
            .map(|op| op.parse::<i32>().unwrap()))
        .collect::<Vec<[i32; 2]>>();

    return operations;
}


fn _parse1(filename: impl AsRef<Path>) -> i32 {

    let operations = _parse_file(filename);

    let result = operations.iter()
        .map(|operation| operation[0] * operation[1])
        .sum::<i32>();

    return result;
}

fn _parse2b(filename: impl AsRef<Path>) -> i32 {

    let operations = _parse_file2b(filename);

    let result = operations.iter()
        .map(|operation| operation[0] * operation[1])
        .sum::<i32>();

    return result;
}

fn _parse2(filename: impl AsRef<Path>) -> i32 {

    let operations = _parse_file2(filename);

    let result = operations.iter()
        .map(|operation| operation[0] * operation[1])
        .sum::<i32>();

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output() {
        let result = _parse2b("src/input");
        println!("done: {result}");
        assert!(false);
    }
}