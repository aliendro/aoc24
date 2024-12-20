use std::{collections::VecDeque, fs, iter::repeat};

fn main() {
    let content = fs::read_to_string("data/inputs/09").expect("Failed to read from input file");

    let part01_solution = part_one(&content);

    println!("Part01 solution: {part01_solution}");

    let part02_solution = part_two(&content);

    println!("Part02 solution: {part02_solution}");
}

fn part_one(input: &str) -> u64 {
    let mut disk = Disk::from(input);
    disk.expand();
    disk.fragment();
    disk.checksum()
}

fn part_two(input: &str) -> u64 {
    let mut disk = Disk::from(input);
    disk.defrag();
    disk.expand();
    disk.checksum()
}

#[derive(Debug, Clone, Copy)]
enum Sector {
    File(u32, u32),
    Free(u32),
}

struct Disk {
    filesystem: Vec<Sector>,
}

impl From<&str> for Disk {
    fn from(input: &str) -> Self {
        let filesystem: Vec<_> = input
            .chars()
            .enumerate()
            .map(|(i, size)| {
                let count = size.to_digit(10).unwrap_or(0);

                match i % 2 {
                    0 => Sector::File((i / 2) as u32, count),
                    _ => Sector::Free(count),
                }
            })
            .collect();

        Self { filesystem }
    }
}

impl Disk {
    fn expand(&mut self) {
        let filesystem: Vec<_> = self
            .filesystem
            .iter()
            .flat_map(|&sector| match sector {
                Sector::File(id, size) => repeat(Sector::File(id, 1))
                    .take(size as usize)
                    .collect::<Vec<Sector>>(),
                Sector::Free(size) => repeat(Sector::Free(1))
                    .take(size as usize)
                    .collect::<Vec<Sector>>(),
            })
            .collect();

        self.filesystem = filesystem;
    }

    fn fragment(&mut self) {
        let mut left = 0;
        let mut right = self.filesystem.len() - 1;

        while left <= right {
            match (self.filesystem[left], self.filesystem[right]) {
                (_, Sector::Free(_)) => right -= 1,
                (Sector::File(_, _), _) => left += 1,
                (_, _) => {
                    self.filesystem.swap(left, right);
                    left += 1;
                    right -= 1;
                }
            }
        }
    }

    fn defrag(&mut self) {
        let mut filesystem = Vec::new();
        let mut vecdeq = VecDeque::from(self.filesystem.clone());

        while let Some(sector) = vecdeq.pop_front() {
            match sector {
                Sector::File(id, size) => {
                    filesystem.push(Sector::File(id, size));
                }
                Sector::Free(mut free_space) => {
                    (0..vecdeq.len()).rev().into_iter().for_each(|position| {
                        if let Sector::File(_, size) = vecdeq[position] {
                            if size <= free_space {
                                filesystem.push(vecdeq[position]);
                                vecdeq.remove(position);
                                vecdeq.insert(position, Sector::Free(size));
                                free_space -= size;
                            }
                        }
                    });
                    if free_space != 0 {
                        filesystem.push(Sector::Free(free_space));
                    }
                }
            }
        }

        self.filesystem = filesystem;
    }

    fn checksum(&self) -> u64 {
        self.filesystem
            .iter()
            .enumerate()
            .map(|(i, &sector)| match sector {
                Sector::File(id, _) => i as u64 * id as u64,
                Sector::Free(_) => 0,
            })
            .sum()
    }
}
