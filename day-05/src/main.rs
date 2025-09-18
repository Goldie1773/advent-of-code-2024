use std::collections::{HashMap, HashSet};

use aoc_common::read_file_manifest;

fn split_input(input: &str) -> (&str, &str) {
    let mut parts = input.split("\r\n\r\n");
    let page_order = parts.next().unwrap_or("");
    let updates = parts.next().unwrap_or("");
    (page_order, updates)
}

fn main() {
    let input = read_file_manifest!("input.txt");
    let (page_order, updates) = split_input(&input);

    let mut order_map: HashMap<u32, Vec<u32>> = HashMap::new();

    for order in page_order.lines() {
        let mut curr_order = order.split("|");
        let key = curr_order.next().unwrap().parse::<u32>().unwrap();
        let value = curr_order.next().unwrap().parse::<u32>().unwrap();
        order_map.entry(key).or_insert_with(Vec::new).push(value);
    }

    let mut total_sum = 0;

    for update in updates.lines() {
        let update: Vec<u32> = update
            .split(",")
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        let middle_num = update[update.len() / 2];

        let mut valid = true;
        let mut stack: HashSet<u32> = HashSet::new();

        for value in update {
            if let Some(values) = order_map.get(&value) {
                for num in values {
                    if stack.contains(num) {
                        valid = false;
                        break;
                    }
                }
            }
            if !valid {
                break;
            }
            stack.insert(value);
        }

        if valid {
            total_sum += middle_num;
        }
    }

    println!("The total sum is {total_sum}");
}
