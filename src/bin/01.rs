use std::{collections::HashMap, fs};

fn main() {
    let content = fs::read_to_string("data/inputs/01").expect("Failed to read from input file");

    let part01_solution = part_one(&content);

    println!("Part1: {part01_solution}");

    let part02_solution = part_two(&content);

    println!("Part2: {part02_solution}");
}

fn part_one(content: &str) -> u32 {
    let (mut left, mut right) = (Vec::<u32>::new(), Vec::<u32>::new());

    content
        .lines()
        .filter_map(|line| {
            let (l, r) = line.split_once("   ")?;
            Some((l.parse().ok()?, r.parse().ok()?))
        })
        .for_each(|(l, r)| {
            left.push(l);
            right.push(r);
        });

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

fn part_two(content: &str) -> u32 {
    let (mut left, mut right) = (Vec::<u32>::new(), HashMap::<u32, u32>::new());

    content
        .lines()
        .filter_map(|line| {
            let (l, r) = line.split_once("   ")?;
            Some((l.parse().ok()?, r.parse().ok()?))
        })
        .for_each(|(l, r)| {
            left.push(l);
            *right.entry(r).or_default() += 1;
        });

    left.iter()
        .map(|k| {
            if let Some(count) = right.get(&k) {
                k * count
            } else {
                0
            }
        })
        .sum()
}
