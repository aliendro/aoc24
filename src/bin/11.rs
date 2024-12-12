use std::{collections::HashMap, fs};

fn main() {
    let content = fs::read_to_string("data/inputs/11").expect("Failed to read from input file");

    let part01_solution = part_one(&content);

    println!("Part01 solution: {part01_solution}");

    let part02_solution = part_two(&content);

    println!("Part02 solution: {part02_solution}");
}

fn part_one(input: &str) -> u64 {
    solve(input, 25)
}

fn part_two(input: &str) -> u64 {
    solve(input, 75)
}

fn solve(input: &str, times: u32) -> u64 {
    let stones: Vec<u64> = input
        .split_ascii_whitespace()
        .filter_map(|stone| stone.parse().ok())
        .collect();

    let mut memo: HashMap<u64, u64> = HashMap::default();

    for stone in stones {
        *memo.entry(stone).or_default() += 1;
    }

    for _ in 0..times {
        let mut map: HashMap<u64, u64> = HashMap::new();

        for (stone, stone_count) in memo.into_iter() {
            match stone {
                0 => {
                    *map.entry(1).or_default() += stone_count;
                }
                stone if (stone.ilog10() + 1) % 2 == 0 => {
                    let stone_digits = stone.ilog10() + 1;

                    let divisor = &10u64.pow(stone_digits / 2);

                    let (l, r) = (stone / divisor, stone % divisor);

                    *map.entry(l).or_default() += stone_count;
                    *map.entry(r).or_default() += stone_count;
                }
                stone => {
                    *map.entry(stone * 2024).or_default() += stone_count;
                }
            }
        }
        memo = map;
    }

    memo.values().sum()
}
