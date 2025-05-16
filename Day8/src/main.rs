use std::fs::read_to_string;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};
use itertools::Itertools;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x: x.try_into().unwrap(), y: y.try_into().unwrap() }
    }
    fn bounds(&self, &height: &i64, &width: &i64) -> bool {
        self.x >=0 && self.y>=0 && self.x < width && self.y < height
    }
}

fn first() -> usize{
    let input = read_to_string("input.txt").expect("Failed to read input file");

    let height :i64 = input.lines().count().try_into().unwrap();
    let width :i64 = input.lines().next().unwrap().len().try_into().unwrap();

    let mut antennas : HashMap<char, Vec<Pos>> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.char_indices() {
            if c == '.' {
                continue;
            }
            antennas.entry(c).or_insert(vec![]).push(Pos::new(i, j));
        }
    }

    let mut antinodes = HashSet::new();

    for possers in antennas.values() {
        for pair in possers.iter().combinations(2) {
            let a = *pair[0];
            let b = *pair[1];
            let diff = a - b;

            let an_1 = a + diff;
            let an_2 = b - diff;

            if an_1.bounds(&height, &width) {
                antinodes.insert(an_1);
            }
            
            if an_2.bounds(&height, &width) {
                antinodes.insert(an_2);
            }
        }
    }
    antinodes.len()
}

fn second() -> usize {
    let input = read_to_string("input.txt").expect("Failed to read input file");

    let height :i64 = input.lines().count().try_into().unwrap();
    let width :i64 = input.lines().next().unwrap().len().try_into().unwrap();

    let mut antennas : HashMap<char, Vec<Pos>> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.char_indices() {
            if c == '.' {
                continue;
            }
            antennas.entry(c).or_insert(vec![]).push(Pos::new(i, j));
        }
    }

    let mut antinodes = HashSet::new();

    for possers in antennas.values() {
        for pair in possers.iter().combinations(2) {
            let a = *pair[0];
            let b = *pair[1];
            let diff = a - b;

            let mut an = a;

            while an.bounds(&height, &width) {
                antinodes.insert(an);
                an = an + diff;
            }
            
            let mut an = b;

            while an.bounds(&height, &width) {
                antinodes.insert(an);
                an = an - diff;
            }
        }
    }
    antinodes.len()
}

fn main() {

    println!("First: {}", first());
    println!("Second: {}", second());
}
