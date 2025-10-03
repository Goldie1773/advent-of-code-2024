use aoc_common::read_file_manifest;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }

    const fn is_in_bounds(self, rows: usize, cols: usize) -> bool {
        self.x >= 0 && self.x < cols as i32 && self.y >= 0 && self.y < rows as i32
    }

    const fn vector_to(self, other: Self) -> Self {
        Self {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }

    const fn translate(self, vector: Self) -> Self {
        Self {
            x: self.x + vector.x,
            y: self.y + vector.y,
        }
    }

    /// Calculate antinodes for a pair of points (Part 1 logic)
    const fn antinodes(self, other: Self) -> (Self, Self) {
        (
            Self {
                x: 2 * self.x - other.x,
                y: 2 * self.y - other.y,
            },
            Self {
                x: 2 * other.x - self.x,
                y: 2 * other.y - self.y,
            },
        )
    }
}

fn parse_antenna_map(grid: &[&str]) -> HashMap<char, Vec<Point>> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| (c, Point::new(x, y)))
        })
        .fold(HashMap::new(), |mut map, (symbol, point)| {
            map.entry(symbol).or_insert_with(Vec::new).push(point);
            map
        })
}

fn find_antinodes_part1(
    node_map: &HashMap<char, Vec<Point>>,
    rows: usize,
    cols: usize,
) -> HashSet<Point> {
    let mut antinodes = HashSet::new();

    for points in node_map.values() {
        for pair in points.iter().combinations(2) {
            let (an1, an2) = pair[0].antinodes(*pair[1]);

            if an1.is_in_bounds(rows, cols) {
                antinodes.insert(an1);
            }
            if an2.is_in_bounds(rows, cols) {
                antinodes.insert(an2);
            }
        }
    }

    antinodes
}

fn find_antinodes_part2(
    node_map: &HashMap<char, Vec<Point>>,
    rows: usize,
    cols: usize,
) -> HashSet<Point> {
    let mut antinodes = HashSet::new();

    for points in node_map.values().filter(|p| p.len() >= 2) {
        
        antinodes.extend(points.iter().copied());

        
        for pair in points.iter().combinations(2) {
            let (p1, p2) = (*pair[0], *pair[1]);
            let vector = p1.vector_to(p2);

            
            let mut current = p1.translate(Point { x: -vector.x, y: -vector.y });
            while current.is_in_bounds(rows, cols) {
                antinodes.insert(current);
                current = current.translate(Point { x: -vector.x, y: -vector.y });
            }

            
            current = p2.translate(vector);
            while current.is_in_bounds(rows, cols) {
                antinodes.insert(current);
                current = current.translate(vector);
            }
        }
    }

    antinodes
}

fn main() {
    let input = read_file_manifest!("input.txt");
    let grid: Vec<&str> = input.lines().collect();
    let (rows, cols) = (grid.len(), grid.first().map_or(0, |row| row.len()));

    let node_map = parse_antenna_map(&grid);
    
    // Part 1: Only immediate antinodes (one on each side)
    let antinodes_part1 = find_antinodes_part1(&node_map, rows, cols);
    println!(
        "Part 1 - The number of unique anti-node locations is {}",
        antinodes_part1.len()
    );

    // Part 2: All collinear antinodes (resonant harmonics)
    let antinodes_part2 = find_antinodes_part2(&node_map, rows, cols);
    println!(
        "Part 2 - The number of unique anti-node locations is {}",
        antinodes_part2.len()
    );
}