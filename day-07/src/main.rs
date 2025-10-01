use aoc_common::read_file_manifest;

const OPS: [fn(u64, u64) -> u64; 2] = [|a, b| a + b, |a, b| a * b];

fn target_in_permutations(nums: &[u64], target: u64) -> bool {
    let n = nums.len();
    if n == 0 {
        return false;
    }
    (0..(1 << (n - 1))).any(|mask| {
        let mut acc = nums[0];
        for i in 0..n - 1 {
            let op = (mask >> i) & 1;
            acc = OPS[op](acc, nums[i + 1]);
        }
        acc == target
    })
}

fn main() {
    let input = read_file_manifest!("input.txt");
    let total_cal_sum: u64 = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(':');
            let target = parts.next()?.trim().parse::<u64>().ok()?;
            let nums: Vec<u64> = parts
                .next()?
                .split_whitespace()
                .filter_map(|x| x.parse::<u64>().ok())
                .collect();
            if target_in_permutations(&nums, target) {
                Some(target)
            } else {
                None
            }
        })
        .sum();

    println!("Total calibration result is {total_cal_sum}");
}