use regex::Regex;
use std::fs::read_to_string;

fn main() {

    let input = read_to_string("input.txt").unwrap();

    let reg = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let res: i32 = reg.captures_iter(&input).map(|cap| {
        let(_, [a,b]) = cap.extract();
        let a = a.parse::<i32>().unwrap();
        let b = b.parse::<i32>().unwrap();
        a * b
    }).sum();

    println!("{}", res);

    let reg2 = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut sum = 0;
    let mut add_up=true;
    for i in reg2.captures_iter(&input) {
        let inst = i.get(0).unwrap().as_str();
        if inst == "do()" {
            add_up=true;
        } else if inst == "don't()" {
            add_up=false;
        } else if add_up {
            let a : i32= i.get(1).unwrap().as_str().parse().unwrap();
            let b : i32= i.get(2).unwrap().as_str().parse().unwrap();
            sum += a * b;
        }
    }
    
    println!("{}", sum);
}
