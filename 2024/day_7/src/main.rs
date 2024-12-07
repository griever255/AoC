use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::fs::File;
use anyhow::Result;

fn open_file() -> io::Result<File> {
    let f = File::open(Path::new("./src/input.txt"))?;
    Ok(f)
}

fn generate_operations(vec: Vec<u64>, part_two: bool) -> Vec<u64> {
    let mut results = Vec::new();

    fn helper(vec: &[u64], current: u64, results: &mut Vec<u64>, part_two: bool) {
        if vec.len() == 1 {
            results.push(current);
            return;
        }

        let next = vec[1];
        
        let add_result = current + next;
        helper(&vec[1..], add_result, results, part_two);

        let mul_result = current * next;
        helper(&vec[1..], mul_result, results, part_two); 

        if part_two {
            let cat_result = format!("{}{}", current, next).parse::<u64>().unwrap();
            helper(&vec[1..], cat_result, results, part_two);
        }
    }
    
    helper(&vec, vec[0], &mut results, part_two);
    results
}

fn main() -> Result<()> {
    let reader = BufReader::new(open_file().unwrap());

    let mut part_one = 0;
    let mut part_two = 0;
    for line in reader.lines() {
        let line = line?;
        let split: Vec<&str> = line.split(":").collect();
        let test_value = split[0].parse::<u64>().unwrap();
        let values: Vec<u64> = split[1].trim().split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
        if generate_operations(values.clone(), false).contains(&test_value) {
            part_one += test_value;
        }

        if generate_operations(values.clone(), true).contains(&test_value) {
            part_two += test_value;
        }
    }   

    println!("Part one solution: {}", part_one);
    println!("Part two solution: {}", part_two);
    Ok(())

}