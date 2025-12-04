use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::fs::File;
use anyhow::Result;

fn open_file() -> io::Result<File> {
    let f = File::open(Path::new("./src/input.txt"))?;
    Ok(f)
}

fn find_max_in_n(batteries: &[u8], n: usize) -> (u8, usize) {
    let max_value = *batteries[..batteries.len() - (n - 1)]
        .iter()
        .max()
        .unwrap();
    let max_index = batteries[..batteries.len() - (n - 1)]
        .iter()
        .position(|&v| v == max_value)
        .unwrap();
    (max_value, max_index)
}

fn find_max_joltage_in_n(batteries: &[u8], n: usize) -> usize {
    let mut max_index;
    let mut batt_values: Vec<u8> = vec![];
    let mut skipped_indexes = 0;
    for i in (1..=n).rev() {
        if skipped_indexes < batteries.len() - i {
            let (max_value, new_max_index) = find_max_in_n(&batteries[skipped_indexes..], i);
            max_index = new_max_index + 1;
            skipped_indexes += max_index;
            batt_values.push(max_value);
        } else {
            let max_value = &batteries[skipped_indexes];
            skipped_indexes += 1;
            batt_values.push(*max_value)
        }
    }

    let mut result: usize = 0;
    eprintln!("{:?}", batt_values);
    for j in batt_values {
        result = result * 10 + j as usize;
    }
    result
}

fn main() -> Result<()> {
    let reader = BufReader::new(open_file().unwrap());
    let mut first_answer: usize = 0;
    let mut second_answer = 0;

    for line in reader.lines() {
        let line = line?;
        let batteries: Vec<u8> = line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
        let joltage_2 = find_max_joltage_in_n(&batteries, 2);
        let joltage_12 = find_max_joltage_in_n(&batteries, 12);
        first_answer += joltage_2;
        second_answer += joltage_12;
    }

    // Part One answer
    eprintln!("part 1 solution is: {:?}", first_answer);

    // Part Two answer
    eprintln!("part_2 solution is: {:?}", second_answer);
    Ok(())
}