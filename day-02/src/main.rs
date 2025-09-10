use aoc_common::read_file_manifest;

fn calcualte_level_safety(report: &[i32]) -> bool {
    // Safe if less than 2 values
    if report.len() < 2 {
        return true;
    }

    let a = report[0];
    let b = report[1];
    
    // Not safe if values are the same as this means the report is neither increasing or decreasing
    if a == b {
        return false;
    }

    // Establish status of increasing vs decreasing
    let increasing = b > a;

    for window in report.windows(2) {
        let (l, r) = (window[0], window[1]);

        // Must remain either increasing or decreasing
        if (r > l) != increasing {
            return false;
        }

        let diff = (r - l).abs();

        if diff < 1 || diff > 3 {
            return false;
        }

    }

    // Report is safe by default after exhausting failure checks
    true
}

fn main() {
    let input_text = read_file_manifest!("input.txt");

    let mut total_levels_safe = 0;

    for line in input_text.lines() {
        let report: Vec<i32> = line.split_whitespace()
                                                        .map(|val| val.parse::<i32>().unwrap())
                                                        .collect();

        if calcualte_level_safety(&report) {
            total_levels_safe += 1;
        }
    }
    
    println!("The total number of safe reports is: {total_levels_safe}")
}