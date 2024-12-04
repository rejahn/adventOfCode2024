use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_values(file: File) -> Vec<Vec<i32>> {
    let mut input: Vec<Vec<i32>> = Vec::new();

    for (i, line) in BufReader::new(file).lines().enumerate() {
        let line_string = line.unwrap(); // Bind the String to a variable
        let string = line_string.split(" ").enumerate();

        if input.len() <= i {
            input.push(Vec::new());
        }

        for (j, str) in string {
            if input[i].len() <= j {
                input[i].push(0);
            }
            input[i][j] = str.parse::<i32>().unwrap();
        }
    }

    input
}

fn first_task(vec: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for sub_vec in vec {
        let mut flag = true;

        let mut iter = sub_vec.iter();
        let mut prev = *iter.next().unwrap();
        let curr = *iter.next().unwrap();
        let increasing = curr > prev;

        if curr == prev {
            flag = false;
        } else {
            let diff = (curr - prev).abs();
            if diff < 1 || diff > 3 {
                flag = false;
            }
        }

        prev = curr;

        for &curr in iter {
            if !flag {
                break;
            }
            let diff = (curr - prev).abs();
            if diff < 1 || diff > 3 {
                flag = false;
                break;
            }
            if increasing && curr <= prev {
                flag = false;
                break;
            } else if !increasing && curr >= prev {
                flag = false;
                break;
            }
            prev = curr;
        }
        if flag {
            sum += 1;
        }
    }
    sum
}

fn second_task(vec: Vec<Vec<i32>>) -> i32 {
  let mut sum = 0;

    for sub_vec in vec.into_iter() {
        let mut safe = false;

        // Try removing each level one by one, including the option of not removing any
        for remove_index in 0..=sub_vec.len() {
            let mut new_vec = Vec::new();
            for (i, &num) in sub_vec.iter().enumerate() {
                if i != remove_index {
                    new_vec.push(num);
                }
            }

            // Initialize the iterator
            let mut iter = new_vec.iter();
            let mut prev = *iter.next().unwrap();
            let next = *iter.next().unwrap();


            let increasing = prev < next;
            let decreasing = prev > next;

            // Check if the initial difference is within allowed range
            let mut safe_sequence = true;
            let mut diff = (next - prev).abs();
            if diff < 1 || diff > 3 {
                safe_sequence = false;
            }

            prev = next;

            for &num in iter {
                if num == prev {
                    safe_sequence = false;
                    break;
                }

                diff = (num - prev).abs();
                if diff < 1 || diff > 3 {
                    safe_sequence = false;
                    break;
                }

                if increasing && num < prev {
                    safe_sequence = false;
                    break;
                }
                if decreasing && num > prev {
                    safe_sequence = false;
                    break;
                }

                prev = num;
            }

            if safe_sequence {
                safe = true;
                break;
            }
        }

        if safe {
            sum += 1;
        }
    }

    sum
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./src/input";
    let file = File::open(file_path)?;

    let res: Vec<Vec<i32>> = read_values(file);
    let res_cp = res.clone();
    let res_01 = first_task(res);
    println!("Result of day 02 task 01: {}", res_01);

    let res_02 = second_task(res_cp);
    println!("Result of day 02 task 01: {}", res_02);

    Ok(())
}
