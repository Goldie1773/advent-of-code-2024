use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter,
};

use aoc_common::read_file_manifest;

fn split_input(input: &str) -> (&str, &str) {
    let mut parts = input.split("\r\n\r\n");
    let page_order = parts.next().unwrap_or("");
    let updates = parts.next().unwrap_or("");
    (page_order, updates)
}

fn reorder_part_two(input: &Vec<u32>, order_map: &HashMap<u32, Vec<u32>>) -> u32 {
    let pages_set: HashSet<u32> = input.iter().cloned().collect();

    let mut adj_list: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();

    for &page in input {
        in_degree.insert(page, 0);
        adj_list.insert(page, Vec::new());
    }

    for &page in input {
        if let Some(dependents) = order_map.get(&page) {
            for &dependent in dependents {
                if pages_set.contains(&dependent) {
                    adj_list.get_mut(&page).unwrap().push(dependent);
                    *in_degree.get_mut(&dependent).unwrap() += 1;
                }
            }
        }
    }

    // Topological sort using Kahn's algorithm
    let mut queue: VecDeque<u32> = VecDeque::new();
    let mut sorted_order: Vec<u32> = Vec::new();

    for (&page, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(page);
        }
    }

    while let Some(current) = queue.pop_front() {
        sorted_order.push(current);

        if let Some(dependents) = adj_list.get(&current) {
            for &dependent in dependents {
                let degree = in_degree.get_mut(&dependent).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(dependent);
                }
            }
        }
    }

    // Return the middle element
    sorted_order[sorted_order.len() / 2]
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
        // let middle_num = update[update.len() / 2]; not needed for part two

        let mut valid = true;
        let mut stack: HashSet<u32> = HashSet::new();

        for value in &update {
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
            stack.insert(*value);
        }

        if valid {
            continue; // Part Two
                      // total_sum += middle_num; // (Part One)
        } else {
            // continue; // Part One)
            total_sum += reorder_part_two(&update, &order_map) // For part Two
        }
    }

    println!("The total sum is {total_sum}");
}
