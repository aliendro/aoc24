fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|level| level.parse::<u32>().expect("Failed to parse level"))
        .collect::<Vec<_>>()
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
                            .filter(|(report_index, _)| *report_index != i)
                            .map(|(_, level)| *level)
                            .collect::<Vec<_>>(),
                    )
                })
        })
        .count()
}
