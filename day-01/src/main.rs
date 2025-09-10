use std::{fs::{self}, iter::zip};

fn read_file(file_path: &str) -> String {
    let input_text = fs::read_to_string(
        format!("{}/{}", env!("CARGO_MANIFEST_DIR"), file_path)
    ).expect("Could not read sample.txt");

    input_text
}

fn split_input(input_text: String) -> (Vec<i32>, Vec<i32>) {
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

fn find_total_difference(left: &Vec<i32>, right: &Vec<i32>) -> u32 {
    let total_diff = zip(left.iter(), right.iter())
                            .map(|(l, r)| (l-r).abs())
                            .sum::<i32>()
                            .try_into().unwrap();

    total_diff
}

fn find_similarity_score(left: &Vec<i32>, right: &Vec<i32>) -> u32 {

    let mut similarity_sum = 0;

    for value in left.iter() {
        let freq: i32 = right.iter()
                                .filter(|&r| *r == *value)
                                .count().try_into().unwrap();
        similarity_sum += *value * freq;
    }

    similarity_sum.try_into().unwrap()
}

fn main() {

    let file_name = "input.txt";
    let input_text = read_file(file_name);

    let (left_list, right_list) = split_input(input_text);
    
    // Part One Calculate the total difference between each list

    let total_diff = find_total_difference(&left_list, &right_list);

    println!("Total difference is: {total_diff}");

    // Part Two: Calculate the similarity score between each list

    let similarity_score = find_similarity_score(&left_list, &right_list);

    println!("Similarity score is: {similarity_score}")
}
