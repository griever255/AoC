use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::fs::File;
use anyhow::Result;

fn open_file() -> io::Result<File> {
    let f = File::open(Path::new("./src/input.txt"))?;
    Ok(f)
}


fn main() -> Result<()> {
    let reader = BufReader::new(open_file().unwrap());

    let mut disk_map: Vec<char> = vec![];
    for line in reader.lines() {
        let line = line?;
        disk_map = line.chars().collect();
    }

    let mut uncompressed_map: Vec<String> = vec![];
    let mut free_space;
    let mut index = -1;
    for i in 0..disk_map.len() {
        if i % 2 != 0 {
            free_space = true;
        } else {
            free_space = false;
            index += 1;
        }
        if !free_space {
            for _ in 0..(disk_map[i] as u8 - 48) {
                uncompressed_map.push(index.to_string());
            }
        } else {
            for _ in 0..(disk_map[i] as u8 - 48) {
                uncompressed_map.push(".".to_string());
            }
        }
    }

    let mut compressed_map = uncompressed_map.clone();
    let mut furtherest_rev = 10;
    for i in 0..uncompressed_map.len() {
        if i >= furtherest_rev {
            break
        }
        if uncompressed_map[i] == "." {
            for j in 0..uncompressed_map.len() {
                let rev = uncompressed_map.len()-1-j;
                if uncompressed_map[rev] == "." {
                    continue
                } else {
                    compressed_map[i] = uncompressed_map[rev].clone();
                    uncompressed_map[rev] = ".".to_string();
                    compressed_map[rev] = ".".to_string();
                    furtherest_rev = rev;
                    break
                }
            }
        }
    }

    let mut part_one: u64 = 0;
    for i in 0..compressed_map.len() {
        if compressed_map[i] != "." {
            part_one += i as u64 * compressed_map[i].parse::<u64>().unwrap();
        }
    }

    println!("Part one solution: {}", part_one);

    Ok(())
}