use std::fs;

fn main() {
    let content = fs::read_to_string("data/inputs/02").expect("Failed to read from input file");

    let part01_solution = part_one(&content);

    println!("Part1: {part01_solution}");

    let part02_solution = part_two(&content);

    println!("Part2: {part02_solution}");
}

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .filter_map(|level| level.parse().ok())
        .collect()
}

fn check_safety(line: &Vec<u32>) -> bool {
    let (first, second) = (line[0], line[1]);
    let initial_direction = first < second;

    for (a, b) in line.iter().zip(line.iter().skip(1)) {
        let current_direction = a < b;

        if initial_direction != current_direction {
            return false;
        }

        let height = a.abs_diff(*b);

        if height < 1 || height > 3 {
            return false;
        }
    }

    true
}

pub fn part_one(content: &str) -> usize {
    content.lines().map(parse_line).filter(check_safety).count()
}

pub fn part_two(content: &str) -> usize {
    content
        .lines()
        .map(parse_line)
        .filter(|line| {
            let is_safe = check_safety(line);
            is_safe
                || (0..line.len()).any(|i| {
                    check_safety(
                        &line
                            .iter()
                            .enumerate()
                            .filter_map(|(report_index, level)| {
                                if report_index != i {
                                    Some(*level)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>(),
                    )
                })
        })
        .count()
}
