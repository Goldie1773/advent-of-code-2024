use std::{collections::{HashMap, HashSet}, hash::Hash, ops::{Add, Sub}};
use aoc_common::read_file_manifest;
use itertools::Itertools;

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
        for pair in points.iter().combinations(2) {
            let p1 = pair[0];
            let p2 = pair[1];

            let dist_between = distance_between_points(p2, p1);

            let an1 = *p1 - dist_between;
            let an2 = *p2 + dist_between;

            if an1.is_in_bounds(&grid_width, &grid_len) {
                antinode_map.insert(an1);
            }

            if an2.is_in_bounds(&grid_width, &grid_len) {
                antinode_map.insert(an2);
            }

        }
    }

    println!("The number of unique anti-node locations is {}", antinode_map.len());
}
