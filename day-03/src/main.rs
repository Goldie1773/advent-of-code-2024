use aoc_common::read_file_manifest;
use regex::Regex;

fn main() {
    let input_text = read_file_manifest!("input.txt");
     // Add "don't" to the end to enable uniform regex query
    let input_text = input_text + "don't()";

    let mut enabled = 1;

    let re_p2 = Regex::new(r"(.*?)(don't\(\)|do\(\))").unwrap();
    let re_p1 = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut mult_sum = 0;

    for (_, [group, enabler]) in re_p2.captures_iter(&input_text).map(|m| m.extract()) {
        println!("Previous Sum: {mult_sum}, Enabled : {enabled}");
        println!("Group: {group}, enabler: {enabler}\n");
        if enabled == 1 {
            for (_, [d_one, d_two]) in re_p1.captures_iter(&group).map(|m| m.extract()) {
                mult_sum += d_one.parse::<i32>().unwrap() * d_two.parse::<i32>().unwrap();
            }
        }

        match enabler {
            "do()" => enabled = 1,
            "don\'t()" => enabled = 0,
            _ => panic!("Capture group should only capture 'do()' or 'don't()'")
        }
    }

    println!("Total sum of multiplications is: {mult_sum}");
    // Current Total sum of multiplications is: 105125951 -> answer is too low
    // Current approach is failing 
    // Alternative working approach is to search for mul or do or dont, add mul if enabled then set enabled based on do or dont if not mul
    // Alternative working approach 2 is to use a stack, adding each letter if it is in mul or do or dont and taking action based on stack
}