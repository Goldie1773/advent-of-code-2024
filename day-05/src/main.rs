use aoc_common::read_file_manifest;

fn split_input(input: String) -> (String, String) {
    let mut parts = input.split("\r\n\r\n");
    let page_order = parts.next().unwrap_or("").to_string();
    let updates = parts.next().unwrap_or("").to_string();
    (page_order, updates)
}

fn main() {
    let input = read_file_manifest!("sample.txt");
    let (page_order, updates) = split_input(input);

    // Implement hash map for storing the relationship between the ordering pairs
    // Use a stack to add the inputs and check each key against the value in the map in order
    // Once verified correct capture the middle index
    // Discard updates not in the correct order

    println!("{page_order}, \n{updates}");
}
