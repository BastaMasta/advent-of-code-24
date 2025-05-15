use std::fs::read_to_string;
use std::collections::HashSet;

#[derive(Clone, PartialEq, Hash, Eq, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn turn_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

struct Grid {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new() -> Self {
        let grid = read_to_string("input.txt")
            .expect("Failed to read file")
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let height = grid.len();
        let width = grid[0].len();
        Grid { grid, height, width }
    }

    fn get_pos(&self) -> (usize, usize) {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.grid[i][j] == '^' {
                    return (i, j);
                }
            }
        }
        panic!("guard found");
    }

    fn next_pos(&self, (guardx, guardy): (usize, usize), direction: &mut Direction) -> Option<(usize, usize)> {
       let (nextx, nexty) = match direction {
            Direction::Up => (guardx.checked_sub(1)?, guardy),
            Direction::Down => (guardx + 1, guardy),
            Direction::Left => (guardx, guardy.checked_sub(1)?),
            Direction::Right => (guardx, guardy + 1),
        };

        let next_tile = self.grid.get(nextx).and_then(|row| row.get(nexty))?;

        if next_tile == &'#' {
            direction.turn_right();
            return Some((guardx, guardy));
        }

        Some((nextx, nexty))
    }
}

fn main() {
    let mut grid = Grid::new();
    let (mut guardx, mut guardy) = grid.get_pos();
    let mut direction = Direction::Up;

    let mut path = HashSet::new();
    path.insert((guardx, guardy));

    while let Some((nextx, nexty)) = grid.next_pos((guardx, guardy), &mut direction) {
        
        path.insert((nextx, nexty));

        guardx = nextx;
        guardy = nexty;
    }
    let tot = path.len();
    println!("Total path length: {}", tot);

    // part 2
    // let (mut guardx, mut guardy) = grid.get_pos();
    // let mut direction = Direction::Up;

    // let mut visited = HashSet::new();
    // let mut count = 0;

    // while let Some((nextx, nexty)) = grid.next_pos((guardx, guardy), &mut direction) {
    //     visited.insert((nextx, nexty));

    //     if !visited.contains(&(nextx, nexty)) {
    //         grid.grid[nextx][nexty] = 'X';
    //         if makes_loop(&grid, (nextx, nexty), direction) {
    //             count += 1;
    //         }
    //         grid.grid[nextx][nexty] = '.';
    //     }
    //     (guardx, guardy) = (nextx, nexty);
    // }

    println!("Total loops: {}", part2());

}

pub fn part2() -> usize {
    let mut grid = Grid::new();
    let (mut guard_row, mut guard_col) = grid.get_pos();
    let mut direction = Direction::Up;

    let mut visited = HashSet::new();
    let mut count = 0;

    while let Some((next_row, next_col)) = grid.next_pos((guard_row, guard_col), &mut direction)
    {
        visited.insert((guard_row, guard_col));

        if !visited.contains(&(next_row, next_col)) {
            grid.grid[next_row][next_col] = '#';
            if makes_loop(&grid, (guard_row, guard_col), direction) {
                count += 1;
            }
            grid.grid[next_row][next_col] = '.';
        }

        (guard_row, guard_col) = (next_row, next_col);
    }

    count
}

fn makes_loop(grid: &Grid, (startx, starty): (usize, usize), start_direction: Direction) -> bool {
    let mut visited_obstacles: Vec<(usize, usize, Direction)> = Vec::new();

    let mut direction = start_direction;
    let (mut guard_row, mut guard_col) = (startx, starty);

    while let Some((next_row, next_col)) = grid.next_pos((guard_row, guard_col), &mut direction)
    {
        if (guard_row, guard_col) == (next_row, next_col) {
            if visited_obstacles.contains(&(guard_row, guard_col, direction)) {
                return true;
            }

            visited_obstacles.push((guard_row, guard_col, direction));
        }

        (guard_row, guard_col) = (next_row, next_col);
    }

    false
}