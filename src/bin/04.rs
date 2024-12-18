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
            count += search_from_point(content, Point(row as i32, col as i32));
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

            let is_valid = search_surroundings(content, Point(row as i32, col as i32));

            if is_valid {
                count += 1;
            }
        }
    }
    count
}

#[derive(Clone, Copy, Debug)]
pub struct Point(i32, i32);

impl Point {
    pub fn walk(&self, to: Direction) -> Self {
        match to {
            Direction::North => Point(self.0 - 1, self.1),
            Direction::South => Point(self.0 + 1, self.1),
            Direction::West => Point(self.0, self.1 - 1),
            Direction::East => Point(self.0, self.1 + 1),
            Direction::Northwest => Point(self.0 - 1, self.1 - 1),
            Direction::Northeast => Point(self.0 - 1, self.1 + 1),
            Direction::Southwest => Point(self.0 + 1, self.1 - 1),
            Direction::Southeast => Point(self.0 + 1, self.1 + 1),
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
    if point.0 < 0 || point.1 < 0 {
        return None;
    }

    if let Some(c) = content
        .lines()
        .nth(point.0 as usize)?
        .chars()
        .nth(point.1 as usize)
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
