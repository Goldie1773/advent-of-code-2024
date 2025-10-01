use aoc_common::read_file_manifest;
use rayon::str::ParallelString;
use rayon::iter::ParallelIterator;

const OPS: [fn(u64, u64) -> u64; 3] = [
    |a, b| a + b,
    |a, b| a * b,
    |a, b| concat(a, b)
  ];

fn target_in_permutations(nums: &[u64], target: u64) -> bool {
    let n = nums.len();
    if n == 0 {
        return false;
    }
    let num_gaps = n - 1;
    let total = 3u64.pow(num_gaps as u32);
    (0..total).any(|mask| {
        let mut acc = nums[0];
        let mut m = mask;
        for i in 0..num_gaps {
            let op = (m % 3) as usize;
            acc = OPS[op](acc, nums[i + 1]);
            m /= 3;
        }
        acc == target
    })
}

fn concat(a: u64, b: u64) -> u64 {
    a as u64 * 10u64.pow(b.ilog10() + 1) + b as u64
}

fn main() {
    let input = read_file_manifest!("input.txt");
    let total_cal_sum: u64 = input
        .par_lines()
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
