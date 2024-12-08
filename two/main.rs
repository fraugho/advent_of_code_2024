use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
   let mut file = File::open("list.txt")?;
   let mut contents = String::new();
   file.read_to_string(&mut contents)?;
   
   let safe_count = contents
       .lines()
       .filter(|line| {
           let nums: Vec<i32> = line
               .split_whitespace()
               .map(|num| num.parse::<i32>().unwrap())
               .collect();

           let differences: Vec<i32> = nums.windows(2)
               .map(|w| w[1] - w[0])
               .collect();

           // Check if differences are within valid range
           let valid_diffs = differences.iter()
               .all(|&diff| diff.abs() >= 1 && diff.abs() <= 3);

           // Check if monotonic (all increasing or all decreasing)
           let all_increasing = differences.iter().all(|&diff| diff > 0);
           let all_decreasing = differences.iter().all(|&diff| diff < 0);

           valid_diffs && (all_increasing || all_decreasing)
       })
       .count();

   println!("{}", safe_count);
   Ok(())
}
