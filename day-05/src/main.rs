use std::{collections::HashMap, vec};

use aoc_common::read_file_manifest;

fn split_input(input: String) -> (String, String) {
    let mut parts = input.split("\r\n\r\n");
    let page_order = parts.next().unwrap_or("").to_string();
    let updates = parts.next().unwrap_or("").to_string();
    (page_order, updates)
}

fn main() {
    let input = read_file_manifest!("input.txt");
    let (page_order, updates) = split_input(input);

    let mut order_map: HashMap<String, Vec<String>> = HashMap::new();
    for order in page_order.lines() {
        let mut order = order.split("|");
        let key = order.next().unwrap().to_string();
        let value = order.next().unwrap().to_string();
        if let Some(temp_v) = order_map.get_mut(&key) {
            temp_v.push(value);
        } else {
            order_map.insert(key, vec![value]);
        }
    }

    let mut total_sum = 0;

    for update in updates.lines() {
        let mut valid = true;
        let mut stack: Vec<String> = Vec::new();
        let update = update.split(",");
        let middle_num: Vec<&str> = update.clone().collect();
        let middle_num = middle_num[middle_num.len() / 2];

        'outer: for value in update {
            if let Some(values) = order_map.get(value) {
                for num in values.iter() {
                    if stack.contains(num) == true {
                        valid = false;
                        break 'outer;
                    }
                }
            }
            stack.push(value.to_string());
        }
        if valid == true {
            total_sum += middle_num.parse::<i32>().unwrap_or(0);
        }
    }
    println!("The total sum is {total_sum}");

    // Implement hash map for storing the relationship between the ordering pairs | DONE
    // Use a stack to add the inputs and check each key against the value in the map in order
    // Once verified correct capture the middle index
    // Discard updates not in the correct order
}
