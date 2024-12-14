use std::{
    fs::File, hash::Hash, io::{prelude::*, BufReader}, path::Path, usize
};
use regex::Regex;

fn main() {
    println!("Hello, world!");
}

struct Arcade {
    machines: Vec<Machine>,
}

struct Machine {
    button_a: Pos,
    button_b: Pos,
    prize: Pos,
}

struct Pos {
    x: usize,
    y: usize,
}

fn _parse_file(filename: impl AsRef<Path>) -> Arcade {

    let regex = Regex::new(r"(Button A|Button B|Prize): X(\+|\=)(\d+), Y(\+|\=)(\d+)").unwrap();

    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);

    let lines = buf.lines();

    let machines: Vec<Machine> = Vec::new();
    let machines = lines.enumerate().fold(machines, |mut acc, (index, line)| {   

        let text = line.unwrap();

        if let Some(captures) = regex.captures(&text) {
            let machine_index = index / 4; 

            if index % 4 < 4 {
                let x: usize = captures.get(3).unwrap().as_str().parse().unwrap();
                let y: usize = captures.get(5).unwrap().as_str().parse().unwrap();

                match index % 4 {
                    0 => {
                        acc.push(Machine {
                            button_a: Pos { x, y },
                            button_b: Pos { x: 0, y: 0 },
                            prize: Pos { x: 0, y: 0 },
                        });
                            }
                    1 =>  {
                        let machine = acc.get_mut(machine_index).unwrap();

                        machine.button_b.x = x;
                        machine.button_b.y = y;
                    }
                    2 =>  {
                        let machine = acc.get_mut(machine_index).unwrap();

                        machine.prize.x = x;
                        machine.prize.y = y;
                    }
                    _ => unreachable!()
                }
            }
        }

        acc
    });

    Arcade {
        machines,
    }
}

fn _intersect(p1: &Pos, p2: &Pos, goal: &Pos) -> (f64, f64) {
    let rico_a = p1.y as f64 / p1.x as f64;
    let rico_b = p2.y as f64 / p2.x as f64;
    let offset_b = goal.y as f64 - (rico_b * goal.x as f64);
    let intersect_x = offset_b / (rico_a - rico_b);
    let size_a = intersect_x;
    let size_b = goal.x as f64 - intersect_x;
    let size_a = (size_a / p1.x as f64).round();
    let size_b = (size_b / p2.x as f64).round();

    (size_a, size_b)
}

fn _parse1(filename: impl AsRef<Path>) -> usize {

    let arcade = _parse_file(filename);

    let coins = arcade.machines.iter().filter_map(|machine| {

        let (pushes_a, pushes_b): (f64, f64) = _intersect(&machine.button_a, &machine.button_b, &machine.prize);

        let claw_x = pushes_a as usize * machine.button_a.x + pushes_b as usize * machine.button_b.x;
        let claw_y = pushes_a as usize * machine.button_a.y + pushes_b as usize * machine.button_b.y;

        let has_prize =
            claw_x == machine.prize.x &&
            claw_y == machine.prize.y;

        // println!("{:>3} x [{:>3}, {:>3}] + {:>3} x [{:>3}, {:>3}] => [{:>6}, {:>6}] ? [{:>6}, {:>6}]{} {}",
        //     pushes_a as usize, machine.button_a.x, machine.button_a.y,
        //     pushes_b as usize, machine.button_b.x, machine.button_b.y,
        //     claw_x,
        //     claw_y,
        //     machine.prize.x, machine.prize.y,
        //     if has_prize { " *" } else { if claw_x.abs_diff(machine.prize.x) < (machine.button_a.x / 2) { " ?" } else { "  " } },
        //     pushes_a as usize * 3 + pushes_b as usize);

        if has_prize {
            Some(pushes_a as usize * 3 + pushes_b as usize)
        } else {
            None
        }
        
    })
    .collect::<Vec<usize>>();

    coins.iter().sum::<usize>()
}

fn _parse2(filename: impl AsRef<Path>) -> usize {
    let arcade = _parse_file(filename);

    let coins = arcade.machines.iter().filter_map(|machine| {

        let machine_prize = Pos {
            x: machine.prize.x + 10000000000000,
            y: machine.prize.y + 10000000000000,
        };

        let (pushes_a, pushes_b): (f64, f64) = _intersect(&machine.button_a, &machine.button_b, &machine_prize);

        let claw_x = pushes_a as usize * machine.button_a.x + pushes_b as usize * machine.button_b.x;
        let claw_y = pushes_a as usize * machine.button_a.y + pushes_b as usize * machine.button_b.y;

        let has_prize =
            claw_x == machine_prize.x &&
            claw_y == machine_prize.y;

        // println!("{:>3} x [{:>3}, {:>3}] + {:>3} x [{:>3}, {:>3}] => [{:>6}, {:>6}] ? [{:>6}, {:>6}]{} {}",
        //     pushes_a as usize, machine.button_a.x, machine.button_a.y,
        //     pushes_b as usize, machine.button_b.x, machine.button_b.y,
        //     claw_x,
        //     claw_y,
        //     machine.prize.x, machine.prize.y,
        //     if has_prize { " *" } else { if claw_x.abs_diff(machine.prize.x) < (machine.button_a.x / 2) { " ?" } else { "  " } },
        //     pushes_a as usize * 3 + pushes_b as usize);

        if has_prize {
            Some(pushes_a as usize * 3 + pushes_b as usize)
        } else {
            None
        }
        
    })
    .collect::<Vec<usize>>();

    coins.iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output() {
        // 22140 is too low
        let result1 = _parse1("src/input");
        println!("done 1: {result1}");
        let result2 = _parse2("src/input");
        println!("done 2: {result2}");
        assert!(false);
    }
}