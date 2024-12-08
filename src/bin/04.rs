use std::fs;

fn main() {
    let content = fs::read_to_string("data/inputs/04").expect("Failed to read from input");

    let part01_solution = part_one(&content);

    println!("Part1 solution: {part01_solution}");

    let part02_solution = part_two(&content);

    println!("Part2 solution: {part02_solution}");
}

fn part_one(content: &str) -> u32 {
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

fn part_two(content: &str) -> u32 {
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

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub row: i32,
    pub col: i32,
}

impl Point {
    pub fn walk(&self, to: Direction) -> Self {
        match to {
            Direction::North => Point {
                row: self.row - 1,
                col: self.col,
            },
            Direction::South => Point {
                row: self.row + 1,
                col: self.col,
            },
            Direction::West => Point {
                row: self.row,
                col: self.col - 1,
            },
            Direction::East => Point {
                row: self.row,
                col: self.col + 1,
            },
            Direction::Northwest => Point {
                row: self.row - 1,
                col: self.col - 1,
            },
            Direction::Northeast => Point {
                row: self.row - 1,
                col: self.col + 1,
            },
            Direction::Southwest => Point {
                row: self.row + 1,
                col: self.col - 1,
            },
            Direction::Southeast => Point {
                row: self.row + 1,
                col: self.col + 1,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Northwest,
    Northeast,
    Southwest,
    Southeast,
}

impl Direction {
    pub fn iterator() -> impl Iterator<Item = Direction> {
        [
            Direction::Northwest,
            Direction::North,
            Direction::Northeast,
            Direction::East,
            Direction::Southeast,
            Direction::South,
            Direction::Southwest,
            Direction::West,
        ]
        .iter()
        .copied()
    }
}

pub fn search_from_point(content: &str, start: Point) -> u32 {
    let mut count: u32 = 0;

    for direction in Direction::iterator() {
        if let Some(_) = expand_to_direction(content, start, direction, "XMAS") {
            count += 1;
        }
    }

    count
}

pub fn expand_to_direction(
    content: &str,
    start: Point,
    direction: Direction,
    pattern: &str,
) -> Option<String> {
    let mut s = String::new();

    let mut path = start;

    while let Some(c) = check_and_collect_point(content, path) {
        s.push(c);
        path = path.walk(direction);

        if s.len() == pattern.len() {
            break;
        }
    }

    if s == pattern {
        Some(s)
    } else {
        None
    }
}

fn check_and_collect_point(content: &str, point: Point) -> Option<char> {
    if point.row < 0 || point.col < 0 {
        return None;
    }

    if let Some(c) = content
        .lines()
        .nth(point.row as usize)?
        .chars()
        .nth(point.col as usize)
    {
        return Some(c);
    }

    None
}

pub fn search_surroundings(content: &str, start_point: Point) -> bool {
    let mut diagonal = String::new();

    if let Some(c) = check_and_collect_point(content, start_point.walk(Direction::Northwest)) {
        diagonal.push(c);
    } else {
        return false;
    }

    if let Some(c) = check_and_collect_point(content, start_point.walk(Direction::Southeast)) {
        diagonal.push(c);
    } else {
        return false;
    }

    if diagonal != "MS" && diagonal != "SM" {
        return false;
    }

    diagonal.clear();

    if let Some(c) = check_and_collect_point(content, start_point.walk(Direction::Northeast)) {
        diagonal.push(c);
    } else {
        return false;
    }

    if let Some(c) = check_and_collect_point(content, start_point.walk(Direction::Southwest)) {
        diagonal.push(c);
    } else {
        return false;
    }

    if diagonal != "MS" && diagonal != "SM" {
        return false;
    }

    true
}
