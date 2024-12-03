use day3::solve;
use std::fs;

fn main() {
    let content = fs::read_to_string("input/day3/input").unwrap();

    let part_one = solve(&content, false);
    let part_two = solve(&content, true);

    println!("Part one: {part_one}");
    println!("Part two: {part_two}");
}
