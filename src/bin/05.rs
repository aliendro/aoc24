use std::fs;
use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let content = fs::read_to_string("data/inputs/05").expect("Failed to read from input file");

    let manual = SafetyManual::from(content.as_str());

    let part01_solution = manual.part_one();

    println!("Part1 solution: {part01_solution}");

    let part02_solution = manual.part_two();

    println!("Part2 solution: {part02_solution}");
}

#[derive(Debug)]
pub struct SafetyManual {
    pub rules: Vec<(u32, u32)>,
    pub updates: Vec<Vec<u32>>,
}

impl SafetyManual {
    fn is_sorted(&self, update: &Vec<u32>) -> bool {
        let map: HashMap<u32, usize> = update
            .iter()
            .enumerate()
            .map(|(page_index, &page_number)| (page_number, page_index))
            .collect();

        self.rules.iter().all(|(page_number, preceded_by)| {
            match (map.get(page_number), map.get(preceded_by)) {
                (Some(&x), Some(&y)) => x < y,
                _ => true,
            }
        })
    }

    fn sort_update_line(&self, mut update: Vec<u32>) -> Vec<u32> {
        update.sort_by(|&x, &y| {
            if self.rules.contains(&(x, y)) {
                Ordering::Less
            } else if self.rules.contains(&(y, x)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        update
    }

    pub fn part_one(&self) -> u32 {
        self.updates
            .iter()
            .filter_map(|update| {
                let is_valid = self.is_sorted(&update);

                if is_valid {
                    update.get(update.len() / 2)
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn part_two(&self) -> u32 {
        self.updates
            .iter()
            .filter_map(|update| {
                let is_invalid = !self.is_sorted(&update);

                if is_invalid {
                    let sorted_update = self.sort_update_line(update.clone());
                    Some(sorted_update[sorted_update.len() / 2])
                } else {
                    None
                }
            })
            .sum()
    }
}

impl From<&str> for SafetyManual {
    fn from(input: &str) -> Self {
        let (rules, updates) = input
            .split_once("\n\n")
            .expect("Failed to split input file");

        let rules = rules
            .lines()
            .filter_map(|line| {
                let (l, r) = line.split_once('|')?;

                Some((l.parse().ok()?, r.parse().ok()?))
            })
            .collect();

        let updates = updates
            .lines()
            .map(|line| {
                line.split(',')
                    .filter_map(|page_number| page_number.parse().ok())
                    .collect()
            })
            .collect();

        Self { rules, updates }
    }
}
