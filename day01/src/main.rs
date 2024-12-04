use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_values(file: File) -> Result<(Vec<i32>, Vec<i32>), Box<dyn std::error::Error>> {
    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();

    for line in BufReader::new(file).lines() {
        let string = line?;
        if let Some((left, right)) = string.split_once("   ") {
            left_vec.push(left.parse()?);
            right_vec.push(right.parse()?);
        } else {
            eprintln!("Warning: Line does not match expected format: {}", string);
        }
    }
    Ok((left_vec, right_vec))
}

fn first_task(left_vec: &mut [i32], right_vec: &mut [i32]) {
    left_vec.sort_unstable();
    right_vec.sort_unstable();

    let sum: i32 = left_vec
        .iter()
        .zip(right_vec.iter())
        .map(|(left, right)| (right - left).abs())
        .sum();

    println!("Answer of task 01 of day 01 is: {}", sum);
}

fn second_task(left_vec: &[i32], right_vec: &[i32]) {
    let mut counts = HashMap::new();
    for &val in right_vec {
        *counts.entry(val).or_insert(0) += 1;
    }

    let sum: i32 = left_vec
        .iter()
        .filter_map(|&val| counts.get(&val).map(|&count| count * val))
        .sum();

    println!("Answer of task 02 of day 01 is: {}", sum);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./src/input";
    let file = File::open(file_path)?;
    let (mut left_vec, mut right_vec) = read_values(file)?;

    first_task(&mut left_vec, &mut right_vec);
    second_task(&left_vec, &right_vec);

    Ok(())
}
