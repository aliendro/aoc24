use std::{collections::HashSet, fs};

fn main() {
    let content = fs::read_to_string("data/inputs/06").expect("Failed to read from input file");

    let mut guard = Guard::from(content.as_str());

    let part01_solution = part_one(&mut guard);

    println!("Part01 solution: {part01_solution}");

    // let part02_solution = part_two(&mut guard);

    // println!("Part02 solution: {part02_solution}");
}

fn part_one(guard: &mut Guard) -> usize {
    while let Some(action) = guard.patrol() {
        match action {
            Action::Rotate => guard.rotate(),
            Action::Walk => guard.walk(),
        }
    }

    guard.visited_tiles.len()
}

fn part_two(guard: &mut Guard) -> u32 {
    guard.look_for_obstacles();

    guard.obstacles
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
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
    initial_position: (usize, usize),
    current_position: (usize, usize),
    direction: Direction,
    map: Vec<Vec<char>>,
    visited_tiles: HashSet<(usize, usize)>,
    history: HashSet<(usize, usize, Direction)>,
    obstacles: u32,
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
            obstacles: 0,
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
    fn patrol(&mut self) -> Option<Action> {
        match self.direction {
            Direction::Up => {
                let (row, col) = (
                    self.current_position.0.checked_sub(1),
                    self.current_position.1,
                );

                match self.map.get(row?)?.get(col) {
                    None => None,
                    Some(c) => match c {
                        &'#' => Some(Action::Rotate),
                        _ => Some(Action::Walk),
                    },
                }
            }
            Direction::Right => {
                let (row, col) = (self.current_position.0, self.current_position.1 + 1);

                match self.map.get(row)?.get(col) {
                    None => None,
                    Some(c) => match c {
                        &'#' => Some(Action::Rotate),
                        _ => Some(Action::Walk),
                    },
                }
            }
            Direction::Down => {
                let (row, col) = (self.current_position.0 + 1, self.current_position.1);

                match self.map.get(row)?.get(col) {
                    None => None,
                    Some(c) => match c {
                        &'#' => Some(Action::Rotate),
                        _ => Some(Action::Walk),
                    },
                }
            }
            Direction::Left => {
                let (row, col) = (
                    self.current_position.0,
                    self.current_position.1.checked_sub(1),
                );

                match self.map.get(row)?.get(col?) {
                    None => None,
                    Some(c) => match c {
                        &'#' => Some(Action::Rotate),
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

    // Perform loop detection on current path
    fn is_loop(&mut self) -> bool {
        todo!()
    }

    fn look_for_obstacles(&mut self) {
        todo!()
    }
}
