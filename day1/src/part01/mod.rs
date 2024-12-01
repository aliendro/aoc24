pub fn run(contents: &str) -> u32 {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for line in contents.lines() {
        let mut parsed = line
            .split(' ')
            .filter(|v| *v != "")
            .map(|v| v.parse::<u32>().unwrap());
        let (left, right) = (parsed.next().unwrap(), parsed.next().unwrap());
        left_list.push(left);
        right_list.push(right);
    }

    left_list.sort();
    right_list.sort();

    left_list
        .into_iter()
        .zip(right_list)
        .map(|(left, right)| left.abs_diff(right))
        .reduce(|a, b| a + b)
        .unwrap()
}
