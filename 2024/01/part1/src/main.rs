use std::fs::File;
use std::io::{self, BufRead};

fn calculate_total_distance(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
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

    let result = calculate_total_distance(left_list, right_list);
    println!("Total distance: {}", result);

    Ok(())
}
