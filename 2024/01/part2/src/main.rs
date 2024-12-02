use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn calculate_similarity_score(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut freq_map = HashMap::new();
    for &num in &right {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    left.iter()
        .map(|&num| num * freq_map.get(&num).unwrap_or(&0))
        .sum()
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().expect("Failed to parse number"))
            .collect();
        if numbers.len() == 2 {
            left_list.push(numbers[0]);
            right_list.push(numbers[1]);
        }
    }

    let result = calculate_similarity_score(left_list, right_list);
    println!("Similarity score: {}", result);

    Ok(())
}

