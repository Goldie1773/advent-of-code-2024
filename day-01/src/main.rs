use std::{collections::HashMap, iter::zip};
use aoc_common::{read_file_manifest};

fn split_input(input_text: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    let mut toks = input_text.split_whitespace().map(|s| {
        s.parse::<i32>().expect("invalid integer")
    });

    while let (Some(l), Some(r)) = (toks.next(), toks.next()) {
        left_list.push(l);
        right_list.push(r);
    }

    left_list.sort();
    right_list.sort();

    (left_list, right_list)
}

fn find_total_difference(left: &[i32], right: &[i32]) -> u32 {
    // Calculate the total difference by pairing the smallest number in the left 
    // list with the smallest number in the right list and calcualting the difference
    // repeat this for the next smallest difference and sum the differences together
    let total_diff = zip(left.iter(), right.iter())
                            .map(|(l, r)| (l-r).abs())
                            .sum::<i32>()
                            .try_into().unwrap();

    total_diff
}

fn find_similarity_score(left: &[i32], right: &[i32]) -> u32 {
    // Calculate a total similarity score by adding up each number
    // in the left list after multiplying it by the number of times
    // that number appears in the right list.
    let mut right_counts: HashMap<i32, usize> = HashMap::new();
    for &v in right {
        *right_counts.entry(v).or_insert(0) += 1;
    }

    let total: i64 = left.iter()
                            .map(|&v| v as i64 * (*right_counts.get(&v).unwrap_or(&0) as i64))
                            .sum();

    total.try_into().unwrap()
}

fn main() {

    let input_text = read_file_manifest!("input.txt");

    let (left_list, right_list) = split_input(&input_text);
    
    // Part One Calculate the total difference between each list

    let total_diff = find_total_difference(&left_list, &right_list);
    println!("Total difference is: {total_diff}");

    // Part Two: Calculate the similarity score between each list

    let similarity_score = find_similarity_score(&left_list, &right_list);
    println!("Similarity score is: {similarity_score}")
}
