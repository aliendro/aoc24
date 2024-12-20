use std::fs;

fn main() {
    let content = fs::read_to_string("data/examples/10").expect("Failed to read from input file");

    println!("{content}");

    let part01_solution = "";

    println!("Part01 solution: {part01_solution}");

    let part02_solution = "";

    println!("Part02 solution: {part02_solution}");
}
