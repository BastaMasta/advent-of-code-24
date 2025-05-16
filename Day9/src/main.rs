use std::fs::read_to_string;

fn first() -> usize{
    let inp = read_to_string("input.txt").expect("Failed to read input file");
    let mut currindx = 0;
    let mut memory = Vec::new();
    for (e,i) in inp.char_indices() {
        if e % 2 ==0 {
            for _ in 0..i.to_string().parse().unwrap() {
                memory.push(currindx.to_string());
            }
        } else {
            for _ in 0..i.to_string().parse().unwrap() {
                memory.push(".".to_string());
            }
        }
        currindx += 1;
    }
    let mut bck_indx = memory.len() - 1;
    let mut fwd_indx = 0;

    while fwd_indx <= bck_indx {
        while fwd_indx <= bck_indx && memory[fwd_indx] != "." {
            fwd_indx += 1;
        }
        while bck_indx >= fwd_indx && memory[bck_indx] == "." {
            if bck_indx == 0 { break; }
            bck_indx -= 1;
        }
        if fwd_indx <= bck_indx {
            memory[fwd_indx] = memory[bck_indx].clone();
            if bck_indx == 0 { break; }
            bck_indx -= 1;
            fwd_indx += 1;
        }
    }
    println!("{:?}", memory[0..fwd_indx].to_vec());

    // Checksum

    let mut checksum = 0;
    for i in 0..fwd_indx {
        checksum += memory[i].parse::<usize>().unwrap() * i;
    }
    checksum
}

fn main() {
    println!("{}", first());
}
