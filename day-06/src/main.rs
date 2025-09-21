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
            Direction::Down  => (1, 0),
            Direction::Left  => (0, -1),
            Direction::Up    => (-1, 0),
        }
    }

    fn rotate_right(self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down  => Direction::Left,
            Direction::Left  => Direction::Up,
            Direction::Up    => Direction::Right,
        }
    }
}

fn parse_grid_and_find_guard(input: String, guard: char) -> (Vec<Vec<char>>, (isize, isize)) {
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

fn move_guard(pos: (isize, isize), dir: Direction) -> (isize, isize) {
    let (row, col) = pos;

    let (dr, dc) = dir.offset();
    (row + dr, col + dc)
}

fn main() {
    let input= read_file_manifest!("input.txt");
    let (grid, mut guard_pos) = parse_grid_and_find_guard(input, '^');
    let mut dir = Direction::Up;
    let mut visited: HashSet<(isize, isize)>  = HashSet::new();

    // Logic:
    // If the guard is blocked, rotate 90 degrees right, otherwise the guard moves forward
    // Repeats until the guard has left the area

    loop {
      visited.insert(guard_pos);  
      let (dr, dc) = dir.offset();
      let next_pos = (guard_pos.0 + dr, guard_pos.1 + dc);

      if next_pos.0 < 0 || next_pos.1 < 0
          || next_pos.0 >= grid.len() as isize
          || next_pos.1 >= grid[0].len() as isize
      {
          break;
      }

      let row = next_pos.0 as usize;
      let col = next_pos.1 as usize;
      let cell = grid[row][col];

      // println!("Current position is {guard_pos:?}, next position is next_pos {next_pos:?}, the cell contains {cell}");
      match cell {
        '.' | '^' => {
            guard_pos = move_guard(guard_pos, dir);
            },
        '#' => {
          dir = dir.rotate_right();
        },
        _ => panic!("Grid should only contain '.' or '#'.")
      }
    }

    println!("The number of distinct positions the guard visited is {}", visited.len());
}

