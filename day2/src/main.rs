use std::{
    path::Path,
    fs::File,
    io::{prelude::*, BufReader},
};

fn parse_file(filename: impl AsRef<Path>) -> Vec<Vec<i32>> {
    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);

    return buf.lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let elements = line.split_whitespace();
            let numbers: Vec<i32> = elements
                .map(|e| e.parse().unwrap())
                .collect();

            return numbers;
        })
        .collect();
}

fn _parse1(filename: impl AsRef<Path>) -> usize {

    let numbers = parse_file(filename);

    let distances = numbers
        .iter()
        .map(|line_items| line_items.windows(2)
            .map(|pair|
                pair[1] - pair[0])
            .collect::<Vec<i32>>())
        .filter(|line_distances|
            line_distances.iter().all(|distance| (1..4).contains(distance)) ||
            line_distances.iter().all(|distance| (-3..-0).contains(distance))
        )
        .collect::<Vec<Vec<i32>>>();

    return distances.len();
}

fn _parse2(filename: impl AsRef<Path>) -> usize {

    let numbers = parse_file(filename);

    let line_items_permutated = numbers
        .iter()
        .map(|line_items|
            line_items.iter().enumerate().map(|(outer, _)|
                line_items.iter().enumerate()
                    .filter(|(inner, _)| *inner != outer)
                    .map(|(_, item)| *item)
                    .collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>())
        .collect::<Vec<Vec<Vec<i32>>>>();

    let line_items_distances = line_items_permutated
        .iter()
        .map(|line_items_permutations| line_items_permutations
            .iter()
            .map(|line_items|
                line_items.windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>())
        .collect::<Vec<Vec<Vec<i32>>>>();

    let line_items_safe = line_items_distances
        .iter()
        .map(|line_item_distances|
            line_item_distances.iter().find(|distances|
                distances.iter().all(|distance| (1..4).contains(distance)) ||
                distances.iter().all(|distance| (-3..-0).contains(distance))))
        .collect::<Vec<Option<&Vec<i32>>>>();
    
    let line_items_filtered = line_items_safe
        .iter()
        .filter(|line_items| **line_items != None)
        .map(|line_items| line_items.unwrap())
        .collect::<Vec<&Vec<i32>>>();

    return line_items_filtered.len();
}

fn main() {
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output() {
        let result = _parse2("src/input");
        println!("done: {result}");
        assert!(false);
    }
}