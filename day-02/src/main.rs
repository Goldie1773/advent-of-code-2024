use aoc_common::read_file_manifest;

// Helper: check if a transition satisfies our rules
fn is_valid_transition(prev: i32, curr: i32, increasing: bool) -> bool {
    let diff = (curr - prev).abs();
    diff >= 1 && diff <= 3 && ((curr > prev) == increasing)
}

// Check if report is valid for a given trend, optionally skipping one index
fn check_report(report: &[i32], increasing: bool, skip_index: Option<usize>) -> bool {
    if report.len() < 2 {
        return true;
    }

    let mut prev = None;

    for (i, &val) in report.iter().enumerate() {
        if Some(i) == skip_index {
            continue; // Skip this index if it's the one we're testing removal for
        }

        if let Some(p) = prev {
            if !is_valid_transition(p, val, increasing) {
                return false;
            }
        }
        prev = Some(val);
    }

    true
}

fn calculate_level_safety(report: &[i32]) -> bool {
    // First check if valid without removing anything (both trends)
    if check_report(report, true, None) || check_report(report, false, None) {
        return true;
    }

    // Try removing each element (one at a time) for both trends
    for skip_idx in 0..report.len() {
        if check_report(report, true, Some(skip_idx)) || check_report(report, false, Some(skip_idx))
        {
            return true;
        }
    }

    false
}

fn main() {
    let input_text = read_file_manifest!("input.txt");
    let mut total_levels_safe = 0;

    for line in input_text.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|val| val.parse::<i32>().unwrap())
            .collect();

        if calculate_level_safety(&report) {
            total_levels_safe += 1;
        }
    }

    println!("The total number of safe reports is: {total_levels_safe}")
}
