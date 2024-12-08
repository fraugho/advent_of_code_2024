use std::fs::File;
use std::io::prelude::*;

mod misunderstood;
use misunderstood::*;

fn main() -> std::io::Result<()> {
    solution();
    misunderstood();
    Ok(())
}

fn solution() -> std::io::Result<()> {
    let mut file = File::open("lists.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Collect left and right numbers into separate vectors
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            // Parse left and right numbers
            let left_num: u32 = parts[0].parse().expect("Invalid left number");
            let right_num: u32 = parts[1].parse().expect("Invalid right number");
            left_list.push(left_num);
            right_list.push(right_num);
        }
    }

    // Sort both lists
    left_list.sort();
    right_list.sort();

    // Calculate total distance
    let total_distance: u32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(&a, &b)| (a as i32 - b as i32).abs() as u32)
        .sum();

    println!("Total distance: {}", total_distance);

    Ok(())
}
