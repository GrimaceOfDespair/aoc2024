use std::{
    fs::File, io::{prelude::*, BufReader}, path::Path
};


fn main() {
    println!("Hello, world!");
}

struct Grid {
    entries: Vec<Entry>,
}

struct Entry {
    result: i64,
    operators: Vec<Number>,
}

struct Number {
    text: String,
    value: i64
}

fn _parse_file(filename: impl AsRef<Path>) -> Grid {

    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);

    let entries = buf.lines().map(|line|
        line.unwrap().split_once(':')
            .map(|(p1, p2)|
                Entry {
                    result: p1.parse().unwrap(),
                    operators: p2.split(' ')
                        .filter(|c|
                            !c.is_empty())
                        .map(|c|
                            Number {
                                text: c.to_string(),
                                value: c.parse().unwrap() })
                        .collect::<Vec<Number>>()
                })
            .unwrap())
            .collect::<Vec<Entry>>();

    Grid {
        entries,
    }
}

fn _expand_operations(entry: &Entry) -> i64 {
    let init = vec![entry.operators[0].value];
    let operations = entry.operators.iter().skip(1).fold(init, |result: Vec<i64>, operand|
        result.iter()
            .map(|r|
                [r * operand.value, r + operand.value])
            .flatten()
            .collect::<Vec<i64>>());

    let result = operations.iter().any(|operation|
        *operation == entry.result);

    if result { return entry.result } else { return 0 };
}

fn concat_operands(o1: i64, o2: i64) -> i64 {
    let concatenated = o1.to_string() + &o2.to_string();
    concatenated.parse().unwrap()
}

fn _expand_operations_with_concat(entry: &Entry) -> i64 {
    let init = vec![entry.operators[0].value];
    let operations = entry.operators.iter().skip(1).fold(init, |result: Vec<i64>, operand|
        result.iter()
            .map(|r|
                [r * operand.value, r + operand.value, concat_operands(*r, operand.value)])
            .flatten()
            .collect::<Vec<i64>>());

    let result = operations.iter().any(|operation|
        *operation == entry.result);

    if result { return entry.result } else { return 0 };
}

fn _parse1(filename: impl AsRef<Path>) -> i64 {
    let grid = _parse_file(filename);

    let valid_operations = grid.entries.iter().map(|entry|
        _expand_operations(entry))
        .collect::<Vec<i64>>();

    let count: i64 = valid_operations.iter().sum();

    count as i64
}

fn _parse2(filename: impl AsRef<Path>) -> i64 {
    let grid = _parse_file(filename);

    let valid_operations = grid.entries.iter().map(|entry|
        _expand_operations_with_concat (entry))
        .collect::<Vec<i64>>();

    let count: i64 = valid_operations.iter().sum();

    count as i64
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