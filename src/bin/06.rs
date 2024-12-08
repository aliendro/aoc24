use std::fs;

fn main() {
    let content = fs::read_to_string("data/inputs/06").expect("Failed to read from input file");

    let part01_solution = part_one(&content);

    println!("Part01 solution: {part01_solution}");
    let part02_solution = part_one(&content);

    println!("Part02 solution: {part02_solution}");
}

fn part_one(content: &str) -> u32 {
    todo!();
}

fn part_two(content: &str) -> u32 {
    todo!();
}
