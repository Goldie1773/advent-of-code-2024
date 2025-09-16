use aoc_common::read_file_manifest;

fn check_xmas(
    grid: &Vec<Vec<char>>,
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    rows: isize,
    cols: isize,
) -> bool {
    let word = ['X', 'M', 'A', 'S'];
    for k in 0..4 {
        let nx = x + dx * k;
        let ny = y + dy * k;
        if nx < 0
            || nx >= rows
            || ny < 0
            || ny >= cols
            || grid[nx as usize][ny as usize] != word[k as usize]
        {
            return false;
        }
    }
    true
}

fn main() {
    let input = read_file_manifest!("input.txt");

    // Parse input into a 2D grid
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    let directions: Vec<(isize, isize)> = vec![
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (1, -1),  // down-left
        (-1, 1),  // up-right
        (-1, -1), // up-left
    ];

    let mut xmas_count = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 'X' {
                // Check each direction from this 'X'
                for &(dx, dy) in &directions {
                    if check_xmas(
                        &grid,
                        i as isize,
                        j as isize,
                        dx,
                        dy,
                        rows as isize,
                        cols as isize,
                    ) {
                        xmas_count += 1;
                    }
                }
            }
        }
    }

    println!("The total number of times 'XMAS' appears is: {xmas_count}");
}
