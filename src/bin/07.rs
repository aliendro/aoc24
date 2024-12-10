use std::fs;

fn main() {
    let content = fs::read_to_string("data/inputs/07").expect("Failed to read from input file");

    let part01_solution = part_one(parse(&content));

    println!("Part01 solution: {part01_solution}");

    let part02_solution = part_two(parse(&content));

    println!("Part02 solution: {part02_solution}");
}

fn part_one(input: Vec<(u64, Vec<u64>)>) -> u64 {
    input
        .iter()
        .filter(|(result, operands)| solve(*result, operands))
        .map(|(result, _)| result)
        .sum()
}

fn part_two(input: Vec<(u64, Vec<u64>)>) -> u64 {
    input
        .iter()
        .filter(|(result, operands)| solve_concat(*result, operands))
        .map(|(result, _)| result)
        .sum()
}

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .filter_map(|line| {
            let (result, operands) = line.split_once(':')?;
            let result: u64 = result.parse().ok()?;
            let operands = operands
                .split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect();

            Some((result, operands))
        })
        .collect()
}

fn solve(result: u64, operands: &[u64]) -> bool {
    operands
        .iter()
        .skip(1)
        .fold(vec![operands[0]], |operands, &operand| {
            operands
                .iter()
                .flat_map(|&res| vec![res + operand, res * operand])
                .collect()
        })
        .contains(&result)
}

fn solve_concat(result: u64, operands: &[u64]) -> bool {
    operands
        .iter()
        .skip(1)
        .fold(vec![operands[0]], |operands, &operand| {
            operands
                .iter()
                .flat_map(|&res| vec![res + operand, res * operand, concat_operand(res, operand)])
                .collect()
        })
        .contains(&result)
}

fn concat_operand(a: u64, b: u64) -> u64 {
    a * u64::from(10 as u64).pow(b.ilog10() + 1) + b as u64
}
