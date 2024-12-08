use std::fs::File;
use std::io::prelude::*;

//I misunderstood the first day advent of code being a list of lists
pub fn misunderstood() -> std::io::Result<()> {
    let mut file = File::open("lists.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Creates an array of number list
    let list_of_num_list: Vec<String> = contents
        .lines()
        .flat_map(|line| line.split_whitespace())
        .map(String::from)
        .collect();
    
    let answer: u32 = list_of_num_list.chunks(2).map(|chunk| {
        // seperates 5 digit number to array of five digits
        let mut minuend: Vec<i8> = chunk[0]
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i8)
            .collect();
        minuend.sort();

        // seperates 5 digit number to array of five digits
        let mut subtrahend: Vec<i8> = chunk[1]
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i8)
            .collect();
        subtrahend.sort();
        
        // subtracts each digit and get sum of difference
        let sum_of_difference: u32 = minuend
            .iter()
            .zip(subtrahend.iter())
            .map(|pair| (pair.0 - pair.1).abs() as u32)
            .sum::<u32>();

        sum_of_difference
    }).sum();

    println!("{}", answer);
    Ok(())
}
