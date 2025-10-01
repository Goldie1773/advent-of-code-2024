use aoc_common::read_file_manifest;
use regex::Regex;

fn target_in_permutations(nums: &[u64], target: u64) -> bool {
    let n = nums.len();
    if n == 0 {
        return false;
    }
    let ops = vec![|a, b| a + b, |a, b| a * b];

    // There are 2^(n-1) operator combinations
    for mask in 0..(1 << (n - 1)) {
        let mut acc = nums[0];
        for i in 0..n - 1 {
            let op = (mask >> i) & 1;
            acc = ops[op](acc, nums[i + 1]);
        }
        if acc == target {
          return true;
        } else {
          continue;
        }
    }
    return false
}

fn main() {
    let input = read_file_manifest!("input.txt");
    
    let re = Regex::new(r"(?<target>[0-9]+): (?<num_stream>.+)").unwrap();
    let mut total_cal_sum = 0;

    for line in input.lines() {
      let caps = re.captures(line).unwrap();
      let target: &u64= &caps["target"].parse::<u64>().unwrap();
      let nums: Vec<u64> = caps["num_stream"].split(" ").map(|x| x.parse::<u64>().unwrap()).collect();

      if target_in_permutations(&nums, *target) {
        total_cal_sum += target
      } else {
        continue;
      }

    }
    println!("Total calibration result is {total_cal_sum}");
}
