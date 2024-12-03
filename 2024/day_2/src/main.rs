use std::cmp::Ordering::{Less, Equal, Greater};
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::fs::File;
use anyhow::Result;

fn open_file() -> io::Result<File> {
    let f = File::open(Path::new("./src/input.txt"))?;
    Ok(f)
}

fn check_safe(levels: Vec<i32>) -> bool {
    let mut increasing: Option<bool> = None;
    let mut prev_value: Option<i32> = None;
    let mut safe = true;
    for value in levels {
        if let Some(prev) = prev_value {
            if (value - prev).abs() > 3 {
                safe = false;
                break;
            }
            if let Some(inc) = increasing {
                match value.cmp(&prev) {
                    Less => { 
                        if inc == true {
                            safe = false;
                            break;
                        } else {
                            prev_value = Some(value);
                        }
                    },
                    Equal => {
                        safe = false;
                        break
                    },
                    Greater => { 
                        if inc == false {
                            safe = false;
                            break;
                        } else {
                            prev_value = Some(value);
                        }
                    },
                }
            } else {
                match value.cmp(&prev) {
                    Less => { 
                        increasing = Some(false);
                        prev_value = Some(value);
                    },
                    Equal => {
                        safe = false;
                        break
                    },
                    Greater => { 
                        increasing = Some(true);
                        prev_value = Some(value);
                    },
                }
            }
        } else {
            prev_value = Some(value);
        }
    }
    safe
}

fn main() -> Result<()> {
    let reader = BufReader::new(open_file().unwrap());

    let mut safe_lines = 0;
    let mut safe_lines_one_removed = 0;
    for line in reader.lines() {
        let line = line?;
        let levels: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let safe = check_safe(levels.clone());
        if safe {
            safe_lines += 1;
        } else {
            for i in 0..levels.len() {
                let mut recheck: Vec<i32> = levels.clone();
                recheck.remove(i);
                let safe = check_safe(recheck);
                if safe {
                    safe_lines_one_removed += 1;
                    break;
                }
            }
        }
    }

    println!("Part 1 answer: {}", safe_lines);
    println!("Part 2 answer: {}", safe_lines + safe_lines_one_removed);
    Ok(())
}