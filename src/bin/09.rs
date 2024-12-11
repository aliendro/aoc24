use std::fs;

fn main() {
    let content = fs::read_to_string("data/inputs/09").expect("Failed to read from input file");

    let part01_solution = part_one(&content);

    println!("Part01 solution: {part01_solution}");

    let part02_solution = "";

    println!("Part02 solution: {part02_solution}");
}

fn part_one(input: &str) -> u64 {
    let mut disk = get_disk_blocks(input);
    let disk = sort_disk(&mut disk);
    disk.iter()
        .enumerate()
        .map(|(id, &size)| id as u64 * size as u64)
        .sum()
}

fn get_disk_blocks(input: &str) -> Vec<Option<u32>> {
    let mut blocks: Vec<Option<u32>> = Vec::new();
    let mut id: u32 = 0;
    for (i, size) in input.chars().enumerate() {
        for _ in '0'..size {
            if i % 2 == 0 {
                blocks.push(Some(id));
            } else {
                blocks.push(None);
            }
        }
        if i % 2 == 0 {
            id += 1;
        }
    }

    blocks
}

fn sort_disk(disk: &mut Vec<Option<u32>>) -> Vec<u32> {
    let (mut l, mut r) = (0, disk.len() - 1);

    while l <= r {
        if disk[r].is_none() {
            r -= 1;
        } else if disk[l].is_some() {
            l += 1;
        } else {
            disk.swap(l, r);
            l += 1;
            r -= 1;
        }
    }

    disk.iter().filter_map(|&num| num).collect()
}
