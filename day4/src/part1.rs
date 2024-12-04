use day4::{search_from_point, Point};

pub fn solve(content: &str) -> u32 {
    let mut count = 0;
    for (row, line) in content.lines().enumerate() {
        for (col, _) in line.chars().enumerate() {
            count += search_from_point(
                content,
                Point {
                    row: row as i32,
                    col: col as i32,
                },
            );
        }
    }
    count
}
