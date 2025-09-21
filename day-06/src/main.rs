use std::collections::HashSet;

use aoc_common::read_file_manifest;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn offset(self) -> (isize, isize) {
        match self {
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
        }
    }

    fn rotate_right(self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
}

fn parse_grid_and_find_guard(input: &str, guard: char) -> (Vec<Vec<char>>, (isize, isize)) {
    // Using String rather than &str so input is consumed by the function and can be dropped after being parsed into the grid
    let mut grid = Vec::new();
    let mut guard_pos = None;

    for (row_idx, line) in input.lines().enumerate() {
        let row: Vec<char> = line.chars().collect();
        if let Some(col_idx) = row.iter().position(|&c| c == guard) {
            guard_pos = Some((row_idx as isize, col_idx as isize));
        }
        grid.push(row);
    }

    (grid, guard_pos.expect("Grid will always contain a guard."))
}

fn is_in_bounds(pos: (isize, isize), rows: isize, cols: isize) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < rows && pos.1 < cols
}

fn main() {
    let (grid, mut guard_pos) = parse_grid_and_find_guard(&read_file_manifest!("input.txt"), '^');
    let mut dir = Direction::Up;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    // Logic:
    // If the guard is blocked, rotate 90 degrees right, otherwise the guard moves forward
    // Repeats until the guard has left the area

    let grid_rows = grid.len() as isize;
    let grid_cols = grid[0].len() as isize;

    loop {
        visited.insert(guard_pos);
        let (dr, dc) = dir.offset();
        let next_pos = (guard_pos.0 + dr, guard_pos.1 + dc);

        if !is_in_bounds(next_pos, grid_rows, grid_cols) {
            break;
        }

        let cell = grid[next_pos.0 as usize][next_pos.1 as usize];

        // println!("Current position is {guard_pos:?}, next position is next_pos {next_pos:?}, the cell contains {cell}");
        match cell {
            '.' | '^' => guard_pos = next_pos,
            '#' => dir = dir.rotate_right(),
            _ => unreachable!("Grid should only contain '.' or '#'."),
        }
    }

    println!(
        "The number of distinct positions the guard visited is {}",
        visited.len()
    );
}
