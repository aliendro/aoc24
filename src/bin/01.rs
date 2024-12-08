use std::{collections::HashMap, fs};

fn main() {
    let content = fs::read_to_string("data/inputs/01").expect("Failed to read from input file");

    let part01_solution = part_one(&content);

    println!("Part1: {part01_solution}");

    let part02_solution = part_two(&content);

    println!("Part2: {part02_solution}");
}

fn part_one(content: &str) -> u32 {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in content.lines() {
        let mut parsed = line
            .split(' ')
            .filter(|v| *v != "")
            .map(|v| v.parse::<u32>().unwrap());
        let (l, r) = (parsed.next().unwrap(), parsed.next().unwrap());
        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

fn part_two(content: &str) -> u32 {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_map: HashMap<u32, u32> = HashMap::new();

    for line in content.lines() {
        let mut parsed = line
            .split(' ')
            .filter(|v| *v != "")
            .map(|v| v.parse::<u32>().unwrap());
        let (left, right) = (parsed.next().unwrap(), parsed.next().unwrap());
        left_list.push(left);
        *right_map.entry(right).or_default() += 1;
    }

    left_list
        .iter()
        .map(|k| {
            if let Some(count) = right_map.get(&k) {
                k * count
            } else {
                0
            }
        })
        .sum()
}
