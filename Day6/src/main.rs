use std::fs::read_to_string;

fn main() {
    let mut grid = read_to_string("input.txt").expect("Failed to read file").lines().map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    let (mut curx, mut cury) = (0,0);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '^' {
                curx = i;
                cury = j;
            }
        }
    }
    mover(&mut grid, curx, cury, 'U');
    let tot = grid.iter().map(|row| row.into_iter().filter(|&c| *c == 'X').count()).sum::<usize>();
    println!("Total: {}", tot);
}

fn mover(grid: &mut Vec<Vec<char>>, curx: usize, cury: usize, direction: char){
    let (mut x, mut y, mut dir) = (curx, cury, direction);
    let mut motion_flag = true;
    while motion_flag {
        let (nextx, nexty) = match dir {
            'U' => (x - 1, y),
            'D' => (x + 1, y),
            'L' => (x, y - 1),
            'R' => (x, y + 1),
            _ => (x, y),
        };
        if nextx >= grid.len() || nexty >= grid[0].len() || grid[nextx][nexty] == '#' || nextx < 0 || nexty < 0 {
            if match_turn(direction, &x, &y, &grid) == '#' || oor(direction, &x, &y, &grid.len(), &grid[0].len()) {
                motion_flag = false
            }
            grid[x][y] = 'X';
            dir = turn(dir);
            
            
        } else {
            grid[x][y] = 'X';
            grid[nextx][nexty] = '^';
            x = nextx;
            y = nexty;
        }
    }
}
fn turn(direction: char) -> char {
    match direction {
        'U' => 'R',
        'D' => 'L',
        'L' => 'U',
        'R' => 'D',
        _ => direction,
    }
}

fn match_turn(direction: char, x: &usize, y: &usize, grid : &Vec<Vec<char>>) -> char {
    if oor(direction, x, y, &grid.len(), &grid[0].len()) {
        return '#';
    }
    let next = match direction {
        'U' => 'R',
        'D' => 'L',
        'L' => 'U',
        'R' => 'D',
        _ => direction,
    };
    let (nextx, nexty) = match next {
        'U' => (x - 1, *y),
        'D' => (x + 1, *y),
        'L' => (*x, y - 1),
        'R' => (*x, y + 1),
        _ => (*x, *y),
    };
    grid[nextx][nexty]
}

fn oor(direction: char, x: &usize, y: &usize, x_lim: &usize, y_lim: &usize) -> bool {
    let (nextx, nexty) = match direction {
        'U' => (x - 1, *y),
        'D' => (x + 1, *y),
        'L' => (*x, y - 1),
        'R' => (*x, y + 1),
        _ => (*x, *y),
    };
    if nextx >= *x_lim || nexty >= *y_lim || nextx < 0 || nexty < 0 {
        return true;
    }
    false
}
