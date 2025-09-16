use aoc_common::read_file_manifest;
use regex::Regex;

fn main() {
    let input_text = read_file_manifest!("input.txt");

    let re = Regex::new(r"mul\(([0-9]+,[0-9]+)\)|(don't\(\))|(do\(\))").unwrap();
    let mut enabled = 1;
    let mut mult_sum = 0;

    for (_, [capt]) in re.captures_iter(&input_text).map(|m| m.extract()) {
        match capt {
            "do()" => enabled = 1,
            "don\'t()" => enabled = 0,
            _ => {
                let nums: Vec<i32> = capt.split(",").map(|n| n.parse::<i32>().unwrap()).collect();
                if enabled == 1 {
                    mult_sum += nums[0] * nums[1];
                }
            }
        }
    }

    println!("Total sum of multiplications is: {mult_sum}");
}
