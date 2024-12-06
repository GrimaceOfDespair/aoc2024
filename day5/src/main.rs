use std::{
    collections::HashMap, fs::File, io::{prelude::*, BufReader}, path::Path
};

fn main() {
    println!("Hello, world!");
}

fn _parse_file(filename: impl AsRef<Path>) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {

    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);
    let mut lines = buf.lines().into_iter();

    let rules = lines.by_ref()
        .take_while(|line|
            !line.as_ref().unwrap().is_empty())
        .map(|line|
            line.unwrap()
                .split_once("|")
                .map(|(p1, p2)|
                    (p1.parse().unwrap(), p2.parse().unwrap()))
                .unwrap())
        .collect::<Vec<(i32, i32)>>();

    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for (p1, p2) in rules {
        rules_map.entry(p1)
            .or_default()
            .push(p2);
    }

    let pages = lines
        .map(|line|
            line.unwrap().split(",")
                .map(|c|
                    c.parse().unwrap())
            .collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    return (rules_map, pages);
}

fn _parse1(filename: impl AsRef<Path>) -> i32 {

    let (rules, pages) = _parse_file(filename);

    let updates = pages.iter().map(|update_pages|
        update_pages
            .iter()
            .enumerate()
            .map(|(source_page_index, source_page)|
                (*source_page, if !rules.contains_key(source_page) { true } else { rules[source_page].iter()
                    .map(|target_page|
                        update_pages.iter().position(|update_page| update_page == target_page))
                    .all(|page2_index|
                        page2_index.is_none() || page2_index.unwrap() > source_page_index) } ))
            .collect::<Vec<(i32, bool)>>())
        .collect::<Vec<Vec<(i32,bool)>>>();

    let filtered_updates = updates
        .iter()
        .filter(|update_pages|
            update_pages.iter().all(|update_page|
                update_page.1))
        .map(|update_pages|
            update_pages[(update_pages.len() - 1) / 2].0)
        .collect::<Vec<i32>>();

    return filtered_updates.iter().sum();
}

fn _fix_update(rules: &HashMap<i32, Vec<i32>>, update: &Vec<(i32, bool)>) -> Vec<(i32, bool)> {

    let error_page1 = update.iter()
        .position(|page|
            !page.1)
        .unwrap();

    let error_page2 = update.iter()
        .position(|page|
            rules[&update[error_page1].0].contains(&page.0))
        .unwrap();

    let mut fixed_update: Vec<i32> = update.iter().map(|page| page.0).collect::<Vec<i32>>();
    fixed_update.swap(error_page1, error_page2);

    let validated_pages = _validate_pages(&fixed_update, rules);

    if validated_pages.iter().all(|page| page.1) {
        return validated_pages;
    }

    return _fix_update(rules, &validated_pages);
}

fn _parse2(filename: impl AsRef<Path>) -> i32 {

    let (rules, pages) = _parse_file(filename);

    let updates = pages.iter()
        .map(|update_pages|
            _validate_pages(update_pages, &rules))
            .filter(|update_page|
                update_page.iter().any(|p| !p.1))
        .collect::<Vec<Vec<(i32,bool)>>>();

    let fixed_updates = updates.iter()
        .map(|update|
            _fix_update(&rules, &update))
        .collect::<Vec<Vec<(i32,bool)>>>();

    let filtered_updates = fixed_updates
        .iter()
        .map(|update_pages|
            update_pages[(update_pages.len() - 1) / 2].0)
        .collect::<Vec<i32>>();

    return filtered_updates.iter().sum();
}

fn _validate_pages(update_pages: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Vec<(i32, bool)> {
    update_pages
        .iter()
        .enumerate()
        .map(|(source_page_index, source_page)|
            (*source_page, _is_valid_page(rules, source_page, update_pages, source_page_index) ))
        .collect::<Vec<(i32,bool)>>()
}

fn _is_valid_page(rules: &HashMap<i32, Vec<i32>>, source_page: &i32, update_pages: &Vec<i32>, source_page_index: usize) -> bool {
    if !rules.contains_key(source_page) { true } else { rules[source_page].iter()
        .map(|target_page|
            update_pages.iter().position(|update_page| update_page == target_page))
        .all(|page2_index|
            page2_index.is_none() || page2_index.unwrap() > source_page_index) }
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