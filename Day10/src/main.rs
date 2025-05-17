use std::fs::read_to_string;

struct  Grid {
    height: usize,
    width: usize,
    grid: Vec<Vec<u8>>
}

fn first() {
    let grid = read_to_string("input.txt").unwrap()
        .lines()
        .map(|line|
            line.chars()
                .map(|x|
                    x.to_string()
                        .parse::<u8>()
                        .unwrap())
                .collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    let map = Grid {
        height: grid.len(),
        width: grid.first().unwrap().iter().len(),
        grid,
    };

    let mut tot = 0;
    for i in 0..map.height {
        for j in 0..map.width {
            if map.grid[i][j] == 9 {
                let mut visited: Vec<(usize, usize)> = Vec::new();
                tot += get_trails((i, j), 9, &map.grid, &mut visited);
            }
        }
    }
    println!("{}", tot);

}

fn get_trails((row, col): (usize, usize), height: u8, map: &Vec<Vec<u8>>, visited: &mut Vec<(usize, usize)>) -> usize{
    if height == 0 && !visited.contains(&(row, col)) {
        visited.push((row, col));
        return 1;
    }
    let mut tot = 0;
    if row > 0 && map[row-1][col] + 1== height {
        tot += get_trails((row-1, col), height - 1, map, visited);
    }
    if col > 0 && map[row][col-1] + 1== height {
        tot += get_trails((row, col-1), height - 1, map, visited);
    }
    if row +  1< map.len() && map[row+1][col] + 1== height {
        tot += get_trails((row+1, col), height - 1, map, visited);
    }
    if col + 1< map.first().unwrap().len() && map[row][col+1] + 1== height {
        tot += get_trails((row, col+1), height - 1, map, visited);
    }
    
    tot
}

fn main() {
    first();
    second();
}

fn second() {
    let grid = read_to_string("input.txt").unwrap()
        .lines()
        .map(|line|
            line.chars()
                .map(|x|
                    x.to_string()
                        .parse::<u8>()
                        .unwrap())
                .collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    let map = Grid {
        height: grid.len(),
        width: grid.first().unwrap().iter().len(),
        grid,
    };

    let mut tot = 0;
    for i in 0..map.height {
        for j in 0..map.width {
            if map.grid[i][j] == 9 {
                let mut visited: Vec<(usize, usize)> = Vec::new();
                tot += count_trails((i, j), 9, &map.grid, &mut visited);
            }
        }
    }
    println!("{}", tot);
}

fn count_trails ((x, y):(usize, usize), height: u8, map: &Vec<Vec<u8>>, visited: &mut Vec<(usize, usize)>) -> usize {
    if height == 0 {
        return 1;
    }
    
    let mut tot = 0;
    
    if x > 0 && map[x-1][y] + 1 == height {
        tot+=count_trails((x-1, y), height - 1, map, visited);
    }
    if y > 0 && map[x][y-1] + 1 == height {
        tot+=count_trails((x, y-1), height - 1, map, visited);
    }
    if x + 1 < map.len() && map[x+1][y] + 1 == height {
        tot+=count_trails((x+1, y), height - 1, map, visited);
    }
    if y + 1 < map.first().unwrap().len() && map[x][y+1] + 1 == height {
        tot+=count_trails((x, y+1), height - 1, map, visited);   
    }
    
    tot
}
