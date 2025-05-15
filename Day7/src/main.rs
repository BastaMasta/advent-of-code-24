use std::fs::read_to_string;


fn main() {
    let mut summer = 0;
    let mut summer_concat = 0;

    for line in read_to_string("input.txt").unwrap().lines() {
        let (lhs, rhs) = line.split_once(':').unwrap();
        let lhs = lhs.parse::<usize>().unwrap();
        let nums : Vec<usize> = rhs.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let (start, rest) = nums.split_first().unwrap();
        if calc(&lhs, &start, &rest) {
            summer += lhs;
        }
        if calc_concat(&lhs, &start, &rest) {
            summer_concat += lhs;
        }
    }
    println!("The sum of all valid lhs is {}", summer);
    println!("The sum of all valid lhs with concat is {}", summer_concat);
}

fn calc(lhs: &usize,start: &usize, nums: &[usize]) -> bool {
    if nums.len() == 0 {
        return lhs==start;
    }
    if start > lhs {
        return false;
    }

    let (first, rest) = nums.split_first().unwrap();
     return calc(lhs, &(start*first), rest) || calc(lhs, &(start+first), rest)
    
}

fn calc_concat(lhs: &usize,start: &usize, nums: &[usize]) -> bool {
    if nums.len() == 0 {
        return lhs==start;
    }
    if start > lhs {
        return false;
    }

    let (first, rest) = nums.split_first().unwrap();
     return calc_concat(lhs, &(start*first), rest) || calc_concat(lhs, &(start+first), rest) || calc_concat(lhs, &concatter(&start,&first), rest)
    
}

fn concatter(lhs: &usize, rhs: &usize) -> usize {
    let mut lhs = lhs.to_string();
    let rhs = rhs.to_string();
    lhs.push_str(&rhs);
    lhs.parse::<usize>().unwrap()
}