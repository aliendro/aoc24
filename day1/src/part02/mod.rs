use std::collections::HashMap;

pub fn run(contents: &str) -> u32 {
    let mut left_map: HashMap<u32, u32> = HashMap::new();
    let mut right_map: HashMap<u32, u32> = HashMap::new();

    for line in contents.lines() {
        let mut parsed = line
            .split(' ')
            .filter(|v| *v != "")
            .map(|v| v.parse::<u32>().unwrap());
        let (left, right) = (parsed.next().unwrap(), parsed.next().unwrap());

        *left_map.entry(left).or_default() += 1;
        *right_map.entry(right).or_default() += 1;
    }

    left_map
        .into_iter()
        .map(|(k, _v)| {
            if let Some(count) = right_map.get(&k) {
                k * count
            } else {
                0
            }
        })
        .reduce(|a, b| a + b)
        .unwrap()
}
