use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(file: File) -> String {
    let mut count = 0;
    let reader = BufReader::new(file);

    // Define the regex to match "mul(...anything...)".
    let re = Regex::new(r"mul\((.*?)\)").unwrap();

    for line in reader.lines() {
        let line = line.unwrap(); // Unwrap the line result safely
        for capture in re.captures_iter(&line) {
            if let Some(matched) = capture.get(1) {
                
                let numbers = capture.get(1);


                println!("Matched: {}", matched.as_str());
            }
        }
    }

    println!("Count: {}", count);

    "NA".to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./src/input";
    let file = File::open(file_path)?;

    let res = read_input(file);
    // let res_cp: Vec<Vec<i32>> = res.clone();
    // let res_01 = first_task(res);
    // println!("Result of day 02 task 01: {}", res_01);

    // let res_02 = second_task(res_cp);
    // println!("Result of day 02 task 01: {}", res_02);

    Ok(())
}
