use std::{
    collections::HashMap, fs::File, io::{prelude::*, BufReader}, path::Path
};

fn _parse_file(filename: impl AsRef<Path>) -> Vec<u8> {

    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);
    let files = buf.lines().map(|line|
        line.unwrap().chars()
            .map(|c|
                c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>())
        .flatten()
        .collect::<Vec<u8>>();

    files
}

fn main() {
    println!("Hello, world!");
}

fn _parse1(filename: impl AsRef<Path>) -> usize {

    let files = _parse_file(filename);

    let mut head = 0;
    let mut tail = files.len() - 1;
    let mut hash = 0usize;
    let mut gap_size = 0;
    let mut tail_size = files[tail];
    let mut file_pointer = 0;

    while head < tail {
        let (file_id, file_size) = if head % 2  == 0{
            let file_id = (head + 1) / 2;
            let file_size = files[head];
            let file_data = (file_id, file_size);

            head += 1;

            file_data
        } else {

            if gap_size == 0 {
                gap_size = files[head];
            }

            let file_id = tail / 2;

            if tail_size < gap_size {
                let file_data = (file_id, tail_size);

                gap_size -= tail_size;
                tail -= 2;
                tail_size = files[tail];

                file_data
            } else {
                let file_data = (file_id, gap_size);

                tail_size -= gap_size;
                gap_size = 0;
                head += 1;

                file_data
            }
        };

        let file_pointer_end = file_pointer as usize + file_size as usize;
        let multiply = (file_pointer..file_pointer_end).sum::<usize>();
        //println!("file {}: {:>2}..{:>2} ({:>2}) => {:>3}", file_id, file_pointer, file_pointer_end, multiply, file_id as usize * multiply);
        hash += file_id as usize * multiply;
        file_pointer += file_size as usize;
    }

    let file_id = (head + 1) / 2;
    let file_pointer_end = file_pointer as usize + tail_size as usize;
    let multiply = (file_pointer..file_pointer_end).sum::<usize>();
    //println!("file {}: {:>2}..{:>2} ({:>2}) => {:>3}", file_id, file_pointer, file_pointer_end, multiply, file_id as usize * multiply);
    hash += file_id as usize * multiply;

    println!();

    hash
}

struct Plug {
    file_id: usize,
    file_index: usize,
    file_size: usize,
}

struct Gap {
    index: usize,
    size: usize,
    plugs: Vec<Plug>,
    available: usize,
}

fn _parse2(filename: impl AsRef<Path>) -> usize {

    let mut files = _parse_file(filename);

    let mut file_index = files.len() - 1;
    let mut gaps: Vec<Gap> = Vec::new();
    let mut moved_files: HashMap<usize, usize> = HashMap::new();
    let mut gap_max_file_index = 0;

    while file_index != 0 {

        let file_size = files[file_index] as usize;
        let file_id = file_index / 2;
        
        let mut gap_list_index = gaps.iter()
            .position(|gap|
                file_size <= gap.available &&
                gap.index < file_index);

        if gap_list_index.is_none() {
            let gap_file_index = files.iter()
                .enumerate()
                .skip(1)
                .step_by(2)
                .position(|(index, gap)|
                    file_size <= *gap as usize &&
                    index > gaps.len() * 2 &&
                    index < file_index);

            if gap_file_index.is_some() {

                let next_gap_file_index = gaps.len();
                let target_gap_file_index = gap_file_index.unwrap();

                //println!("{next_gap_file_index} => {target_gap_file_index}");

                for gap_index in next_gap_file_index..=target_gap_file_index {

                    let index = gap_index * 2 + 1;
                    let size = files[index] as usize;
    
                    gaps.push(Gap {
                        index,
                        size,
                        available: size,
                        plugs: Vec::new(),
                    });
                }
                
                gap_list_index = Some(gaps.len() - 1);
                gap_max_file_index = target_gap_file_index.max(gap_max_file_index);
            };
        }

        if gap_list_index.is_some() {

            let gap = &mut gaps[gap_list_index.unwrap()];

            //println!("Moving file {file_id} (size: {file_size}) blocks from [{file_index}] to [{}]", gap.index);

            moved_files.insert(file_index, file_size);

            gap.available -= file_size;
            gap.plugs.push(Plug {
                file_id,
                file_index,
                file_size,
            });
        }
        
        file_index -= 2;
    }

    // for gap in &gaps {
    //     println!("gap {} with plugs {:?}", gap.index, gap.plugs.iter().map(|plug| plug.file_id).collect::<Vec<usize>>());
    // }

    let mut hash = 0;
    let mut file_pointer = 0;
    for file_index in 0..(files.len()) {

        match file_index % 2 {
            0 => {
                let file_id = (file_index + 1) / 2;
                let file_size = files[file_index];
                let file_pointer_end = file_pointer + file_size as usize;
                let multiply = (file_pointer..file_pointer_end).sum::<usize>();

                if !moved_files.contains_key(&file_index) {
                    //print!("{}", file_id.to_string().repeat(file_size as usize));
                    hash += file_id * multiply
                } else {
                    //print!("{}", ".".repeat(file_size as usize));
                }
                //println!("file [{file_id}] * [{multiply:>3}] (size {file_size}) = {}", file_id * multiply);

            },
            1 => {
                let gap_index = file_index / 2;
                let mut gap_pointer = file_pointer;

                if gap_index < gaps.len() {
                    for plug in &gaps[gap_index].plugs {
                        let file_id = plug.file_id;
                        let file_size = plug.file_size; 
                        let gap_pointer_end = gap_pointer + file_size as usize;
                        let multiply = (gap_pointer..gap_pointer_end).sum::<usize>();
    
                        //print!("{}", file_id.to_string().repeat(file_size as usize));
                        //println!("gap  [{file_id}] * [{multiply:>3}] (size {file_size}) = {}", file_id * multiply);
                        gap_pointer += file_size;
    
                        hash += file_id * multiply;
                    }
    
                    //print!("{}", ".".repeat(gaps[gap_index].available));
                } else {
                    //print!("{}", ".".repeat(files[file_index] as usize));
                }
            },
            _ => unreachable!(),
        };

        file_pointer += files[file_index] as usize;
    }

    // let file_id = (head + 1) / 2;
    // let file_pointer_end = head_pointer as usize + files[head] as usize;
    // let multiply = (file_pointer..file_pointer_end).sum::<usize>();
    // println!("file {}: {:>2}..{:>2} ({:>2}) => {:>3}", file_id, file_pointer, file_pointer_end, multiply, file_id as usize * multiply);
    // hash += file_id as usize * multiply;

    println!();

    hash
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