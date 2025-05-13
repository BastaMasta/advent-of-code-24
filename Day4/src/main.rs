use std::fs::read_to_string;

fn main() {
    let inp = read_to_string("input.txt").expect("Failed to read input file");
    let grid = inp.lines().map(|l| l.chars().collect::<Vec<char>>() ).collect::<Vec<Vec<char>>>();
    let width = grid[0].len();
    let height = grid.len();
    // Part 1
    // Fucking Bruteforcing it
    let mut tot = 0;
    for i in 0..height {
        for j in 0..width {
            if i<height-3{
                if grid[i][j] == 'X'{
                    if grid[i+1][j] == 'M'{
                        if grid[i+2][j] == 'A'{
                            if grid[i+3][j] == 'S'{
                                tot += 1;
                            }
                        }
                    }
                }
                else if grid[i][j] == 'S'{
                    if grid[i+1][j] == 'A'{
                        if grid[i+2][j] == 'M'{
                            if grid[i+3][j] == 'X'{
                                tot += 1;
                            }
                        }
                    }
                }
            }
            if j<width-3{
                if grid[i][j] == 'X'{
                    if grid[i][j+1] == 'M'{
                        if grid[i][j+2] == 'A'{
                            if grid[i][j+3] == 'S'{
                                tot += 1;
                            }
                        }
                    }
                }
                else if grid[i][j] == 'S'{
                    if grid[i][j+1] == 'A'{
                        if grid[i][j+2] == 'M'{
                            if grid[i][j+3] == 'X'{
                                tot += 1;
                            }
                        }
                    }
                }

            }
            if i<height-3 && j<width-3{
                if grid[i][j] == 'X'{
                    if grid[i+1][j+1] == 'M'{
                        if grid[i+2][j+2] == 'A'{
                            if grid[i+3][j+3] == 'S'{
                                tot += 1;
                            }
                        }
                    }
                }
                else if grid[i][j] == 'S'{
                    if grid[i+1][j+1] == 'A'{
                        if grid[i+2][j+2] == 'M'{
                            if grid[i+3][j+3] == 'X'{
                                tot += 1;
                            }
                        }
                    }
                }
            }
            if i<height-3 && j>2{
                if grid[i][j] == 'X'{
                    if grid[i+1][j-1] == 'M'{
                        if grid[i+2][j-2] == 'A'{
                            if grid[i+3][j-3] == 'S'{
                                tot += 1;
                            }
                        }
                    }
                }
                else if grid[i][j] == 'S'{
                    if grid[i+1][j-1] == 'A'{
                        if grid[i+2][j-2] == 'M'{
                            if grid[i+3][j-3] == 'X'{
                                tot += 1;
                            }
                        }
                    }
                }
            }
            
        }
    }
    println!("Total: {}", tot);

    // Part 2

    let mut tot2 = 0;
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == 'A' && i > 0 && i < height-1 && j > 0 && j < width-1 {
                let ul = grid[i-1][j-1] as u32;
                let ur = grid[i-1][j+1] as u32;
                let dl = grid[i+1][j-1] as u32;
                let dr = grid[i+1][j+1] as u32;
                if ul.abs_diff(dr) == 6 && dl.abs_diff(ur) == 6 {
                    tot2 += 1;
                }
            }
        }
    }
    println!("Total2: {}", tot2);
}