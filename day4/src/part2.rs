use day4::{search_surroundings, Point};

pub fn solve(content: &str) -> u32 {
    let mut count = 0;
    for (row, line) in content.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != 'A' {
                continue;
            }

            let is_valid = search_surroundings(
                content,
                Point {
                    row: row as i32,
                    col: col as i32,
                },
            );

            if is_valid {
                count += 1;
            }
        }
    }
    count
}
