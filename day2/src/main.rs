use day2::{part_one, part_two};
use std::fs;

fn main() {
    let content = fs::read_to_string("input/day2/input").unwrap();

    let solution1 = part_one(&content);

    // let solution2 = part_two(&content);

    println!("{solution1:?}");
    // println!("{solution2:?}");
}
