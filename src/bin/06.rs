use std::{collections::HashSet, fs};

fn main() {
    let content = fs::read_to_string("data/inputs/06").expect("Failed to read from input file");

    let mut guard = Guard::from(content.as_str());

    let part01_solution = part_one(&mut guard);

    println!("Part01 solution: {part01_solution}");

    // let part02_solution = part_two(&guard);

    // println!("Part02 solution: {part02_solution}");
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

fn part_two(guard: &Guard) -> u32 {
    todo!()
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
    steps: u32, // Just for fun
    map: String,
    visited: HashSet<(usize, usize)>,
}

impl From<&str> for Guard {
    fn from(input: &str) -> Self {
        let mut guard = Guard {
            direction: Direction::Up,
            position: (0, 0),
            steps: 1,
            map: input.to_string(),
            visited: HashSet::new(),
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
    fn patrol(&self) -> Option<Action> {
        match self.direction {
            Direction::Up => {
                let (row, _col) = self.position;

                if row == 0 {
                    return None;
                }

                let (row, col) = (self.position.0 - 1, self.position.1);

                match self.map.lines().nth(row)?.chars().nth(col) {
                    Some(c) if c == '#' => Some(Action::Rotate),
                    Some(_c) => Some(Action::Walk),
                    _ => None,
                }
            }
            Direction::Right => {
                let (_row, col) = self.position;

                if col == self.map.lines().nth(0).unwrap().len() - 1 {
                    return None;
                }
                let (row, col) = (self.position.0, self.position.1 + 1);

                match self.map.lines().nth(row)?.chars().nth(col) {
                    Some(c) if c == '#' => Some(Action::Rotate),
                    Some(_c) => Some(Action::Walk),
                    _ => None,
                }
            }
            Direction::Down => {
                let (row, _col) = self.position;

                if row == self.map.lines().nth(0).unwrap().len() - 1 {
                    return None;
                }
                let (row, col) = (self.position.0 + 1, self.position.1);

                match self.map.lines().nth(row)?.chars().nth(col) {
                    Some(c) if c == '#' => Some(Action::Rotate),
                    Some(_c) => Some(Action::Walk),
                    _ => None,
                }
            }
            Direction::Left => {
                let (_row, col) = self.position;

                if col == 0 {
                    return None;
                }
                let (row, col) = (self.position.0, self.position.1 - 1);

                match self.map.lines().nth(row)?.chars().nth(col) {
                    Some(c) if c == '#' => Some(Action::Rotate),
                    Some(_c) => Some(Action::Walk),
                    _ => None,
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
                self.steps += 1;
                self.position = (self.position.0 - 1, self.position.1);
                self.visited.insert(self.position);
            }
            Direction::Right => {
                self.steps += 1;
                self.position = (self.position.0, self.position.1 + 1);
                self.visited.insert(self.position);
            }
            Direction::Down => {
                self.steps += 1;
                self.position = (self.position.0 + 1, self.position.1);
                self.visited.insert(self.position);
            }
            Direction::Left => {
                self.steps += 1;
                self.position = (self.position.0, self.position.1 - 1);
                self.visited.insert(self.position);
            }
        }
    }
}
