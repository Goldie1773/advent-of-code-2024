use std::{collections::{HashMap, HashSet}, hash::Hash, ops::{Add, Sub}};
use aoc_common::read_file_manifest;
use itertools::Itertools;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x: x as i32, y: y as i32 }
    }

    fn is_in_bounds(self, grid_width: &usize, grid_length: &usize) -> bool {
        self.x >= 0 && self.x < *grid_length as i32 && self.y >= 0 && self.y < *grid_width as i32
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// Helper functions for debugging by printing hashset in a sorted order

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }
// struct PointSet<'a>(&'a HashSet<Point>);

// impl<'a> fmt::Display for PointSet<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut points: Vec<_> = self.0.iter().collect();
//         points.sort_by_key(|p| (p.y, p.x));
//         for point in points {
//             writeln!(f, "{point}")?;
//         }
//         Ok(())
//     }
// }

fn distance_between_points(a: &Point, b: &Point) -> Point {
    let diff = *a - *b;
    Point { x: diff.x, y: diff.y }
}

fn main() {
    let input = read_file_manifest!("input.txt");
    let grid: Vec<&str> = input.lines().collect();
    let grid_width = grid.len();
    let grid_len = grid.first().map(|row| row.len()).unwrap_or(0);

    let mut node_map: HashMap<char, Vec<Point>> = HashMap::new();
    let mut antinode_map: HashSet<Point> = HashSet::new();

    for (idx_y, row) in grid.iter().enumerate() {
        for (idx_x, sym) in row.chars().enumerate() {

            if sym != '.' {
                let point = Point::new(idx_x, idx_y);
                node_map.entry(sym).or_insert_with(Vec::new).push(point);
            }
        }
    }

    for (_sym, points) in &node_map {
        
        // Part 2 Only
        if points.len() >= 2 {
            for point in points {
                antinode_map.insert(*point);
            }
        }

        for pair in points.iter().combinations(2) {
            let p1 = pair[0];
            let p2 = pair[1];

            let dist_between = distance_between_points(p2, p1);

            let mut an1 = *p1 - dist_between;
            let mut an2 = *p2 + dist_between;

            while an1.is_in_bounds(&grid_width, &grid_len) || an2.is_in_bounds(&grid_width, &grid_len) {
                if an1.is_in_bounds(&grid_width, &grid_len) {
                    antinode_map.insert(an1);
                }

                if an2.is_in_bounds(&grid_width, &grid_len) {
                    antinode_map.insert(an2);
                }

                an1 = an1 - dist_between;
                an2 = an2 + dist_between;
            }

            

        }
    }

    // println!("{}\n", PointSet(&antinode_map)); // Debugging Purposes

    println!("The number of unique anti-node locations is {}", antinode_map.len());
}
