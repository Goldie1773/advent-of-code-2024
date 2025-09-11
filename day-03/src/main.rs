use aoc_common::read_file_manifest;
use regex::Regex;

fn main() {
    let input_text = read_file_manifest!("input.txt");

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut mult_sum = 0;

    for (_, [d_one, d_two]) in re.captures_iter(&input_text).map(|m| m.extract()) {
        mult_sum += d_one.parse::<i32>().unwrap() * d_two.parse::<i32>().unwrap();
    }

    println!("Total sum of multiplications is: {mult_sum}");
}