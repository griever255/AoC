use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::fs::File;
use anyhow::{anyhow, Result};

fn open_file() -> io::Result<File> {
    let f = File::open(Path::new("./src/input.txt"))?;
    Ok(f)
}

fn main() -> Result<()> {
    let reader = BufReader::new(open_file().unwrap());
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let values: Vec<&str> = line.split_whitespace().collect();

        if let (Ok(val1), Ok(val2)) = 
            (values[0].parse::<i32>(), values[1].parse::<i32>()) {
                list1.push(val1);
                list2.push(val2);
        } else { return Err(anyhow!("Failed to parse two values")) }
    }
    
    list1.sort();
    list2.sort();
    
    let mut sum: i32 = 0;
    let mut similarity: i32 = 0;
    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
        similarity += (list2.iter().filter(|&n| *n == list1[i]).count() as i32) * list1[i];
    }

    // Part One answer
    println!("Part one answer: {}", sum);

    // Part Two answer
    println!("Part two answer: {}", similarity);
    Ok(())
}
