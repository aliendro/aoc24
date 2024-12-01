use std::collections::HashMap;

pub fn run(contents: &str) -> u32 {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_map: HashMap<u32, u32> = HashMap::new();

    for line in contents.lines() {
        let mut parsed = line
            .split(' ')
            .filter(|v| *v != "")
            .map(|v| v.parse::<u32>().unwrap());
        let (left, right) = (parsed.next().unwrap(), parsed.next().unwrap());
        left_list.push(left);
        *right_map.entry(right).or_default() += 1;
    }

    left_list
        .iter()
        .map(|k| {
            if let Some(count) = right_map.get(&k) {
                k * count
            } else {
                0
            }
        })
        .sum()
}
