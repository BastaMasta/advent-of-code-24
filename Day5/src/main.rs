use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {

    let mut page_hash : HashMap<u32, Vec<u32>> = HashMap::new();
    let mut update : Vec<u32> = Vec::new();

    let mut data_flipflop = false;

    for line in read_to_string("input.txt").unwrap().lines() {
        if !data_flipflop {
            if line.is_empty() {
                data_flipflop = true;
                continue;
            }
            conditions.push(line.split('|').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>());
            let cond = line.split('|').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            let curr = page_hash.entry(cond[0]).or_insert(vec![cond[1]]);
            if !curr.contains(&cond[1]) {
                curr.push(cond[1]);
            }
        
        }        
    }
    println!("Conditions: {:?}", page_hash);
}
