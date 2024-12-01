pub fn run(contents: &str) -> u32 {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in contents.lines() {
        let mut parsed = line
            .split(' ')
            .filter(|v| *v != "")
            .map(|v| v.parse::<u32>().unwrap());
        let (l, r) = (parsed.next().unwrap(), parsed.next().unwrap());
        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}
