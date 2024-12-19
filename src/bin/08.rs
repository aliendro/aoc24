use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::{Add, Sub},
};

fn main() {
    let content = fs::read_to_string("data/inputs/08").expect("Failed to read from input file");

    let map = Map::from(content.as_str());

    let part01_solution = map.emit_signals().len();

    println!("Part01 solution: {part01_solution}"); // 252

    let part02_solution = "";

    println!("Part02 solution: {part02_solution}");
}

#[derive(Debug, Clone, Copy, Hash, Eq)]
struct Point(i32, i32);

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<char>>,
    antennas: HashMap<char, Vec<Point>>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

        let antennas = grid
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(y, &cell)| match cell {
                        '.' => None,
                        _ => Some((cell, Point(x as i32, y as i32))),
                    })
            })
            .fold(HashMap::new(), |mut antennas, (frequency, position)| {
                antennas
                    .entry(frequency)
                    .and_modify(|v: &mut Vec<Point>| v.push(position))
                    .or_insert(vec![position]);
                antennas
            });

        Self { grid, antennas }
    }
}

impl Map {
    fn get_point(&self, Point(x, y): Point) -> Option<char> {
        self.grid.get(x as usize)?.get(y as usize).copied()
    }

    fn get_antenna_pairs(&self) -> Vec<(Point, Point)> {
        self.antennas
            .values()
            .flat_map(|locations| {
                locations
                    .iter()
                    .flat_map(|&p1| locations.iter().map(move |&p2| (p1, p2)))
                    .filter(|(p1, p2)| p1 != p2)
            })
            .collect()
    }

    fn calculate_antinodes(&self, p1: Point, p2: Point) -> Vec<Point> {
        [p1 + (p1 - p2), p2 + (p2 - p1)]
            .into_iter()
            .filter(move |&antinode| self.get_point(antinode).is_some())
            .collect()
    }

    fn emit_signals(&self) -> HashSet<Point> {
        let antenna_pairs: Vec<(Point, Point)> = self.get_antenna_pairs();
        let mut antinodes = HashSet::new();

        for (antenna1, antenna2) in antenna_pairs {
            let antinodes_for_pair = self.calculate_antinodes(antenna1, antenna2);
            antinodes.extend(antinodes_for_pair);
        }

        antinodes
    }
}
