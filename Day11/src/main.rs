use std::{collections::HashMap, mem, fs::read_to_string};

use itertools::Itertools;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let stones: HashMap<usize, usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .counts();

    count_stones(stones, 25)
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let stones: HashMap<usize, usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .counts();

    count_stones(stones, 75)
}

fn count_stones(mut stones: HashMap<usize, usize>, blinks: usize) -> usize {
    let mut new: HashMap<usize, usize> = HashMap::new();

    for _ in 0..blinks {
        for (stone, count) in stones.drain() {
            if stone == 0 {
                *new.entry(1).or_default() += count;
            } else {
                let digit_count = stone.ilog10() + 1;
                if digit_count % 2 == 0 {
                    let left = stone / 10usize.pow(digit_count / 2);
                    let right = stone % 10usize.pow(digit_count / 2);
                    *new.entry(left).or_default() += count;
                    *new.entry(right).or_default() += count;
                } else {
                    *new.entry(stone * 2024).or_default() += count;
                }
            }
        }

        mem::swap(&mut stones, &mut new);
    }

    stones.values().sum()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}