use std::fs::read_to_string;
use std::{collections::HashSet, cmp::Ordering};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (orderings, updates ) = input.split_once("\r\n\r\n").unwrap();

    let orderings : HashSet<(usize, usize)> = orderings.lines()
        .map(|line| (line[0..2].parse().unwrap(), line[3..].parse().unwrap()))
        .collect();

    let compare = |x: &usize, y: &usize| !orderings.contains(&(*y, *x));

    let mut count = 0;
    for update in updates.lines() {
        let update : Vec<usize> = update.split(',').map(|x| x.parse().unwrap()).collect();

        if update.is_sorted_by(compare) {
            count += update[update.len()/2];
        }
    }
    println!("{}", count);

    let compare2 = |x: &usize, y : &usize| {
        if orderings.contains(&(*x, *y)) {
            return Ordering::Less;
        }
        if orderings.contains(&(*y, *x)) {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    };

    let res : usize = updates.lines().map(|update| {
        let mut update: Vec<usize> = update.split(',').map(|x| x.parse().unwrap()).collect();
        if update.is_sorted_by(|a,b| compare2(a,b) != Ordering::Greater) {
            0
        } else {
            update.sort_by(compare2);
            update[update.len()/2]
        }
    }).sum();
    
    
    println!("{}", res);
}
