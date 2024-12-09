use std::{collections::HashSet, fs};

fn main() {
    let content = fs::read_to_string("data/examples/06").expect("Failed to read from input file");

    let mut guard = Guard::from(content.as_str());

    let part01_solution = part_one(&mut guard);

    println!("Part01 solution: {part01_solution}");

    let part02_solution = part_two(&mut guard);

    println!("Part02 solution: {part02_solution}");
}

fn part_one(guard: &mut Guard) -> usize {
    while let Some(action) = guard.patrol() {
        match action {
            Action::Rotate => guard.rotate(),
            Action::Walk => guard.walk(),
        }
    }

    guard.visited.len()
}

fn part_two(guard: &mut Guard) -> u32 {
    while let Some(action) = guard.patrol() {
        match action {
            Action::Rotate => guard.rotate(),
            Action::Walk => guard.walk(),
        }
    }

    guard.obstructions
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
enum Action {
    Walk,
    Rotate,
}

#[derive(Debug)]
struct Guard {
    position: (usize, usize),
    direction: Direction,
    map: String,
    visited: HashSet<(usize, usize)>,
    obstructions: u32,
}

impl From<&str> for Guard {
    fn from(input: &str) -> Self {
        let mut guard = Guard {
            direction: Direction::Up,
            position: (0, 0),
            map: input.to_string(),
            visited: HashSet::new(),
            obstructions: 0,
        };

        for (row, line) in input.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                if char == '^' {
                    guard.position = (row, col);
                    guard.visited.insert(guard.position);
                    return guard;
                }
            }
        }

        guard
    }
}

impl Guard {
    fn patrol(&mut self) -> Option<Action> {
        match self.direction {
            Direction::Up => {
                let (row, col) = (self.position.0.checked_sub(1), self.position.1);

                match self.map.lines().nth(row?)?.chars().nth(col) {
                    None => None,
                    Some(c) => match c {
                        '#' => Some(Action::Rotate),
                        _ => Some(Action::Walk),
                    },
                }
            }
            Direction::Right => {
                let (row, col) = (self.position.0, self.position.1 + 1);

                match self.map.lines().nth(row)?.chars().nth(col) {
                    None => None,
                    Some(c) => match c {
                        '#' => Some(Action::Rotate),
                        _ => Some(Action::Walk),
                    },
                }
            }
            Direction::Down => {
                let (row, col) = (self.position.0 + 1, self.position.1);

                match self.map.lines().nth(row)?.chars().nth(col) {
                    None => None,
                    Some(c) => match c {
                        '#' => Some(Action::Rotate),
                        _ => Some(Action::Walk),
                    },
                }
            }
            Direction::Left => {
                let (row, col) = (self.position.0, self.position.1.checked_sub(1));

                match self.map.lines().nth(row)?.chars().nth(col?) {
                    None => None,
                    Some(c) => match c {
                        '#' => Some(Action::Rotate),
                        _ => Some(Action::Walk),
                    },
                }
            }
        }
    }

    fn rotate(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
        }
    }

    fn walk(&mut self) {
        match self.direction {
            Direction::Up => {
                self.position = (self.position.0 - 1, self.position.1);
                self.visited.insert(self.position);
            }
            Direction::Right => {
                self.position = (self.position.0, self.position.1 + 1);
                self.visited.insert(self.position);
            }
            Direction::Down => {
                self.position = (self.position.0 + 1, self.position.1);
                self.visited.insert(self.position);
            }
            Direction::Left => {
                self.position = (self.position.0, self.position.1 - 1);
                self.visited.insert(self.position);
            }
        }
    }
}
