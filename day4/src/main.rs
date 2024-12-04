mod part1;
mod part2;

use std::fs;

fn main() {
    let content = fs::read_to_string("input/day4/input").expect("Failed to read from input");

    let part_one = part1::solve(&content);
    println!("Part1 solution: {part_one}");

    let part_two = part2::solve(&content);

    println!("Part2 solution: {part_two}");
}
