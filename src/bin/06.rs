use std::{collections::HashSet, fs};

fn main() {
    let content = fs::read_to_string("data/inputs/06").expect("Failed to read from input file");

    let mut guard = Guard::from(content.as_str());

    let part01_solution = part_one(&mut guard);

    println!("Part01 solution: {part01_solution}");

    let part02_solution = part_two(&mut guard);

    println!("Part02 solution: {part02_solution}");
}

fn part_one(guard: &mut Guard) -> usize {
    while let Some(action) = guard.patrol() {
        match action {
            State::Rotate => guard.rotate(),
            _ => guard.walk(),
        }
    }

    guard.visited_tiles.len()
}

fn part_two(guard: &mut Guard) -> u32 {
    let mut obstacles = 0;

    let candidates = guard.visited_tiles.clone();
    let original_map = guard.map.clone();

    for (row, col) in candidates.iter() {
        guard.reset();
        guard.map = original_map.clone();
        guard.map[*row][*col] = '#';

        while let Some(action) = guard.patrol() {
            match action {
                State::Rotate => guard.rotate(),
                State::Walk => guard.walk(),
                State::Loop => {
                    obstacles += 1;
                    break;
                }
            }
        }
    }

    obstacles
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
enum State {
    Walk,
    Rotate,
    Loop,
}

#[derive(Debug)]
struct Guard {
    initial_position: (usize, usize),
    current_position: (usize, usize),
    direction: Direction,
    visited_tiles: HashSet<(usize, usize)>,
    history: HashSet<(usize, usize, Direction)>,
    map: Vec<Vec<char>>,
}

impl From<&str> for Guard {
    fn from(input: &str) -> Self {
        let mut guard = Guard {
            initial_position: (0, 0),
            current_position: (0, 0),
            direction: Direction::Up,
            map: input
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            visited_tiles: HashSet::new(),
            history: HashSet::new(),
        };

        for (row, line) in guard.map.iter().enumerate() {
            for (col, char) in line.iter().enumerate() {
                if char == &'^' {
                    guard.current_position = (row, col);
                    guard.initial_position = (row, col);
                    guard.visited_tiles.insert(guard.current_position);
                    guard.history.insert((row, col, guard.direction.clone()));
                    return guard;
                }
            }
        }

        guard
    }
}

impl Guard {
    fn patrol(&mut self) -> Option<State> {
        match self.direction {
            Direction::Up => {
                let (row, col) = (
                    self.current_position.0.checked_sub(1),
                    self.current_position.1,
                );

                if self.history.contains(&(row?, col, self.direction.clone())) {
                    return Some(State::Loop);
                }

                match self.map.get(row?)?.get(col) {
                    None => None,
                    Some(c) => match c {
                        &'#' => Some(State::Rotate),
                        _ => Some(State::Walk),
                    },
                }
            }
            Direction::Right => {
                let (row, col) = (self.current_position.0, self.current_position.1 + 1);

                if self.history.contains(&(row, col, self.direction.clone())) {
                    return Some(State::Loop);
                }

                match self.map.get(row)?.get(col) {
                    None => None,
                    Some(c) => match c {
                        &'#' => Some(State::Rotate),
                        _ => Some(State::Walk),
                    },
                }
            }
            Direction::Down => {
                let (row, col) = (self.current_position.0 + 1, self.current_position.1);

                if self.history.contains(&(row, col, self.direction.clone())) {
                    return Some(State::Loop);
                }

                match self.map.get(row)?.get(col) {
                    None => None,
                    Some(c) => match c {
                        &'#' => Some(State::Rotate),
                        _ => Some(State::Walk),
                    },
                }
            }
            Direction::Left => {
                let (row, col) = (
                    self.current_position.0,
                    self.current_position.1.checked_sub(1),
                );

                if self.history.contains(&(row, col?, self.direction.clone())) {
                    return Some(State::Loop);
                }

                match self.map.get(row)?.get(col?) {
                    None => None,
                    Some(c) => match c {
                        &'#' => Some(State::Rotate),
                        _ => Some(State::Walk),
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
                self.current_position = (self.current_position.0 - 1, self.current_position.1);
                self.visited_tiles.insert(self.current_position);
                self.history.insert((
                    self.current_position.0,
                    self.current_position.1,
                    self.direction.clone(),
                ));
            }
            Direction::Right => {
                self.current_position = (self.current_position.0, self.current_position.1 + 1);
                self.visited_tiles.insert(self.current_position);
                self.history.insert((
                    self.current_position.0,
                    self.current_position.1,
                    self.direction.clone(),
                ));
            }
            Direction::Down => {
                self.current_position = (self.current_position.0 + 1, self.current_position.1);
                self.visited_tiles.insert(self.current_position);
                self.history.insert((
                    self.current_position.0,
                    self.current_position.1,
                    self.direction.clone(),
                ));
            }
            Direction::Left => {
                self.current_position = (self.current_position.0, self.current_position.1 - 1);
                self.visited_tiles.insert(self.current_position);
                self.history.insert((
                    self.current_position.0,
                    self.current_position.1,
                    self.direction.clone(),
                ));
            }
        }
    }

    fn reset(&mut self) {
        self.current_position = self.initial_position;
        self.direction = Direction::Up;
        self.history.clear();
    }
}
