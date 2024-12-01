use std::fs;

mod part01;
mod part02;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let part01_result = part01::run(&contents);
    let part02_result = part02::run(&contents);

    println!("Part 01 result: {part01_result} \nPart 02 result: {part02_result}");
}
