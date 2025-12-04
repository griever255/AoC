use core::str;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::fs::File;
use anyhow::Result;
use factor::factor_include::factor_include;

fn open_file() -> io::Result<File> {
    let f = File::open(Path::new("./src/input.txt"))?;
    Ok(f)
}

fn test_repeated_twice(test_id: &str) -> bool {
    let length = test_id.len();
    let first_part = &test_id[..length/2];
    let second_part = &test_id[length/2..];
    first_part == second_part
}

fn test_repeated_n(test_id: &str, n: i64) -> bool {
    let parts = test_id.as_bytes().chunks(n.try_into().unwrap()).map(str::from_utf8).collect::<Result<Vec<&str>, _>>().unwrap();
    let first_part = parts[0];
    parts.iter().all(|part| *part == first_part)
}

fn main() -> Result<()> {
    let reader = BufReader::new(open_file().unwrap());
    let mut first_answer = 0;
    let mut second_answer = 0;
    let mut invalid_ids: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line?;
        let ranges: Vec<&str> = line.split(",").collect();
        for range in ranges {
            let (beginning, end) = range.split_once("-")
                                                    .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                                                    .expect("Input is valid");                                                
            for i in beginning..=end {
                let test_id = i.to_string();

                let factors = factor_include(test_id.len().try_into().unwrap());
                for n in &factors[..factors.len() - 1] {
                    if test_repeated_n(&test_id, *n) {
                        eprintln!("Found one: {:?}", test_id);
                        if !invalid_ids.contains(&test_id) {
                            invalid_ids.push(test_id.clone());
                            second_answer += test_id.parse::<usize>().unwrap();
                        }
                    }
                }
                
                if test_id.len() % 2 == 1 {
                    continue
                } else {
                    if test_repeated_twice(&test_id) {
                        first_answer += test_id.parse::<usize>().unwrap(); 
                    }
                }
            }

        }
    }

    // Part One answer
    eprintln!("part 1 solution is: {:?}", first_answer);

    // Part Two answer
    eprintln!("part_2 solution is: {:?}", second_answer);
    Ok(())
}