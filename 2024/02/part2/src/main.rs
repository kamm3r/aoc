use std::fs::File;
use std::io::Read;

fn is_safe_sequence(nums: &[i32]) -> bool {
    if nums.len() < 2 {
        return true;
    }

    let mut direction: Option<i32> = None;
    for window in nums.windows(2) {
        let diff = window[1] - window[0];

        if diff == 0 || diff.abs() > 3 {
            return false;
        }

        match direction {
            None => {
                direction = Some(diff.signum());
            }
            Some(dir) if dir != diff.signum() => {
                return false;
            }
            _ => {}
        }
    }

    true
}

fn is_safe_with_dampener(nums: &[i32]) -> bool {
    if is_safe_sequence(nums) {
        return true; // Already safe.
    }

    for i in 0..nums.len() {
        let mut modified = nums.to_vec();
        modified.remove(i);

        if is_safe_sequence(&modified) {
            return true;
        }
    }
    false
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut count = 0;

    for line in contents.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().expect("Failed to parse number"))
            .collect();

        let result = is_safe_with_dampener(&numbers);
        count += result as u32;
    }
    println!("Safe sequences: {}", count);

}
