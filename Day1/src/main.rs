use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {

    // Part 1
    let mut left_vec: Vec<i32> = Vec::new();
    let mut right_vec: Vec<i32> = Vec::new();

    for line in read_to_string("./src/inputs.txt").unwrap().lines() {
        let mut nums = line.split_whitespace();
        left_vec.push(nums.next().unwrap().parse::<i32>().unwrap());
        right_vec.push(nums.next().unwrap().parse::<i32>().unwrap());
    }
    left_vec.sort();
    right_vec.sort();
    let mut sum = 0;
    for i in 0..left_vec.len() {
        if left_vec[i] < right_vec[i] {
            sum += right_vec[i] - left_vec[i];
        }
        else {
            sum += left_vec[i] - right_vec[i];
        }

    }
    println!("total distance diff: {}", sum);

    // Part 2
    let mut sum2 = 0;
    for i in left_vec {
        sum2 += i * right_vec.clone().into_iter().filter(|x| *x == i).count() as i32;
    }
    println!("total weighted value: {}", sum2);
}
