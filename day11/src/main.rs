use std::{
    collections::HashMap, fs::File, io::{prelude::*, BufReader}, path::Path
};

fn main() {
    println!("Hello, world!");
}

fn _parse_file(filename: impl AsRef<Path>) -> Vec<usize> {

    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);
    let numbers = buf.lines().map(|line|
        line.unwrap().split(' ')
            .map(|s|
                s.parse().unwrap())
            .collect::<Vec<usize>>())
        .flatten()
        .collect::<Vec<usize>>();

    numbers
}

fn split_number(number: usize) -> Vec<usize> {
    if number == 0 {
        vec!(1)
    } else {
        let number_size = number.ilog10() + 1;
        if number_size % 2 == 0 {
            let split = 10u32.pow(number_size / 2) as usize;
            let left = (number / split) as usize;
            let right = number - (left * split);
            vec!(left, right)
        } else {
            vec!(number * 2024)
        }
    }
}

fn _parse1(filename: impl AsRef<Path>) -> usize {
    let numbers = _parse_file(filename);

    let range = 0..25;

    let stones = range.fold(numbers, |acc, _depth| {

        let new_numbers = acc.iter().map(|number| {
            split_number(*number)
        })
        .flatten()
        .collect::<Vec<usize>>();

        //println!("{:?}", new_numbers);

        new_numbers
        
    })
    .iter()
    .map(|x| *x)
    .collect::<Vec<usize>>();
    
    stones.len()
}

#[derive(Clone)]
struct Stone {
    number: usize,
    frequency: usize,
}

fn _parse2(filename: impl AsRef<Path>) -> usize {
    let numbers = _parse_file(filename);

    let number_frequencies = numbers.iter().map(|number| Stone {
        number: *number,
        frequency: 1
    })
    .collect::<Vec<Stone>>();

    let range = 0..75;

    let stones = range.fold(number_frequencies, |acc, _depth| {

        let split_frequencies = acc.iter().map(|frequency| {
            split_number(frequency.number).iter().map(|split|
                Stone {
                    number: *split,
                    frequency: frequency.frequency,
                })
                .collect::<Vec<Stone>>()
        })
        .flatten()
        .collect::<Vec<Stone>>();

        let mut new_frequencies: HashMap<usize, usize> = HashMap::new();
        for frequency in split_frequencies {
            let number = frequency.number;
            if let Some(existing_number) = new_frequencies.get(&number) {
                new_frequencies.insert(number, existing_number + frequency.frequency);
            } else {
                new_frequencies.insert(number, frequency.frequency);
            }
        }

        let new_frequencies = new_frequencies.iter()
            .map(|(number, frequency)|
                Stone {
                    number: *number,
                    frequency: *frequency,
                })
            .collect::<Vec<Stone>>();

        new_frequencies
        
    })
    .iter()
    .map(|x| x.frequency)
    .collect::<Vec<usize>>();

    let result = stones.iter().sum::<usize>();
    
    result
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