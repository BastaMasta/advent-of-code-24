use std::fs::read_to_string;
use crate::MemSeg::{FileSeg, FreeSeg};

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

#[derive(Debug)]
enum MemSeg {
    FreeSeg{size: usize},
    FileSeg{id: usize, size: usize},
}

fn improved_first() -> usize{

    let mut memory: Vec<MemSeg> = Vec::new();
    for (idx, entry) in read_to_string("input.txt").expect("Failed to read input file").trim().char_indices() {
        let size = entry.to_digit(10).unwrap() as usize;
        if idx % 2 == 0 {
            let id = idx/2;
            memory.push(FileSeg{ id, size });
        } else {
            memory.push(FreeSeg{ size });
        }
    }

    let mut clean_memory: Vec<MemSeg> = Vec::new();

    let mut write_idx = 0;
    while write_idx < memory.len() {
        let block = &memory[write_idx];
        match *block {
            FileSeg{ id, size } => {
                clean_memory.push(FileSeg{ id, size });
            },
            FreeSeg{ size } => {
                fill_freespace(&mut memory ,size.clone(), write_idx, &mut clean_memory);
            }
        }
        write_idx += 1;
    }
    checksum(&clean_memory)
}

fn fill_freespace(memory: &mut Vec<MemSeg> ,mut freesize: usize, write_idx: usize, clean_memory: &mut Vec<MemSeg>) {
    let mut read_idx = memory.len() - 1;
    while freesize > 0 && read_idx > write_idx {
        if let FileSeg {id, size: filesize} = memory[read_idx] {
            if filesize <= freesize {
                clean_memory.push(FileSeg{ id, size: filesize });
                freesize -= filesize;
                memory.remove(read_idx);
                read_idx -= 1;
            }
            else {
                clean_memory.push(FileSeg{ id, size: freesize });
                memory[read_idx] = FileSeg{ id, size: filesize - freesize };
                freesize = 0;
            }
        } else {
            read_idx -= 1;
        }
    }
}

fn second() -> usize {
    let mut memory: Vec<MemSeg> = Vec::new();
    for (idx, entry) in read_to_string("input.txt").expect("Failed to read input file").trim().char_indices() {
        let size = entry.to_digit(10).unwrap() as usize;
        if idx % 2 == 0 {
            let id = idx/2;
            memory.push(FileSeg{ id, size });
        } else {
            memory.push(FreeSeg{ size });
        }
    }

    let mut i = memory.len() - 1;

    while i > 0 {
        if let FileSeg{id, size:filesize} = memory[i] {
            let mut insert_idx = 0;
            loop {
                if let FreeSeg { size: freesize } = memory[insert_idx] {
                    if freesize > filesize {
                        memory[i] = FreeSeg { size: filesize };
                        memory[insert_idx] = FileSeg { id, size: filesize };
                        memory.insert(insert_idx+1, FreeSeg { size: freesize - filesize });
                        break;
                    }
                    if filesize == freesize {
                        memory[i] = FreeSeg { size: freesize };
                        memory[insert_idx] = FileSeg { id, size: filesize };
                        break;
                    }
                }
                if insert_idx == i {
                    break;
                }
                insert_idx += 1;
            }
        }
        i-=1;
    }
    checksum(&memory)
}

fn main() {
    println!("{}", improved_first());
    println!("{}", second());
}

fn checksum(memory: &Vec<MemSeg>) -> usize {
    let mut position = 0;
    let mut checksum = 0;
    for block in memory {
        match *block {
            FreeSeg { size } => {
                position += size;
            },
            FileSeg { id, size } => {
                for _ in 0..size {
                    checksum += id * position;
                    position += 1;
                }
            }
        }
    }
    checksum
}