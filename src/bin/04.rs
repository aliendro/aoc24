use std::fs;

fn main() {
    let content = fs::read_to_string("data/inputs/04").expect("Failed to read from input");

    let part01_solution = part_one(&content);

    println!("Part1 solution: {part01_solution}");

    let part02_solution = part_two(&content);

    println!("Part2 solution: {part02_solution}");
}

fn part_one(content: &str) -> u32 {
    let lines: Vec<&str> = content.lines().collect();
    let mut count = 0;
    for (row, line) in content.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != 'X' {
                continue;
            }
            count += search_from_point(&lines, Point(row as i32, col as i32));
        }
    }
    count
}

fn part_two(content: &str) -> u32 {
    let lines: Vec<&str> = content.lines().collect();

    let mut count = 0;
    for (row, line) in content.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != 'A' {
                continue;
            }

            if check_surroundings(&lines, Point(row as i32, col as i32)) {
                count += 1;
            }
        }
    }
    count
}

#[derive(Clone, Copy, Debug)]
struct Point(i32, i32);

impl Point {
    fn walk(&self, to: Direction) -> Self {
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
enum Direction {
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
    fn iterator() -> impl Iterator<Item = Direction> {
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

fn get_char(lines: &[&str], point: Point) -> Option<char> {
    lines
        .get(usize::try_from(point.0).ok()?)?
        .chars()
        .nth(usize::try_from(point.1).ok()?)
}

fn check_pattern(lines: &[&str], start: Point, direction: Direction, pattern: &str) -> bool {
    let mut current = start;

    for expected_char in pattern.chars() {
        match get_char(lines, current) {
            Some(c) if c == expected_char => {
                current = current.walk(direction);
            }
            _ => return false,
        }
    }
    true
}

fn search_from_point(lines: &[&str], start: Point) -> u32 {
    let mut count: u32 = 0;
    let pattern = "XMAS";

    for direction in Direction::iterator() {
        if check_pattern(lines, start, direction, pattern) {
            count += 1;
        }
    }

    count
}

fn check_surroundings(lines: &[&str], start: Point) -> bool {
    let diagonals = [
        (Direction::Northwest, Direction::Southeast),
        (Direction::Northeast, Direction::Southwest),
    ];
    diagonals.iter().all(|(dir1, dir2)| {
        let p1 = get_char(lines, start.walk(*dir1));
        let p2 = get_char(lines, start.walk(*dir2));

        matches!((p1, p2), (Some('M'), Some('S')) | (Some('S'), Some('M')))
    })
}
