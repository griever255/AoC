use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::fs::File;
use anyhow::Result;
use std::collections::HashMap;

fn open_file() -> io::Result<File> {
    let f = File::open(Path::new("./src/input.txt"))?;
    Ok(f)
}

fn main() -> Result<()> {
    let reader = BufReader::new(open_file().unwrap());

    let mut page_orders: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = vec![];
    let mut swap = false;
    for line in reader.lines() {
        let line = line?;
        if line == "" { 
            swap = true;
            continue
        } 
        if !swap {
            let pages: Vec<u32> = line.split("|").map(|p| p.parse::<u32>().unwrap()).collect();
            let key = pages[0];
            let before_page = pages[1];
            let before_pages = page_orders.entry(key).or_insert_with(|| vec![]);
            before_pages.push(before_page);
        } else {
            let update = line.split(",").map(|p| p.parse::<u32>().unwrap()).collect();
            updates.push(update);
        }
    }

    let mut middle_sum = 0;
    let mut incorrect_updates: Vec<Vec<u32>> = vec![];
    for update in updates {
        let mut valid = false;
        'outer: for i in 0..update.len()-1 {
            let before_page = &update[i];
            let after_pages = &update[i+1..];
            for page in after_pages {
                if page_orders.get(&page).unwrap().contains(before_page) {
                    valid = false;
                    // println!("Not valid because {page} is after {before_page}, and should be before {:?}", page_orders.get(&page).unwrap());
                    incorrect_updates.push(update.clone());
                    break 'outer;
                } else {
                    valid = true;
                }
            }
        }
        if valid {
            // println!("The middle of {:?} is {:?}", update, update[update.len() / 2]);
            middle_sum += update[update.len() / 2];
        }
    }

    let mut part_two_sum = 0;
    for update in &incorrect_updates {
        let mut valid = false;
        let mut rearrange = update.clone();
        while !valid {
            'outer: for i in 0..rearrange.len()-1 {
                let before_page = rearrange[i];
                let insert_pos = rearrange.iter().position(|&x| x == before_page).unwrap();
                for j in (i + 1)..rearrange.len() {
                    let page = rearrange[j];
                    if page_orders.get(&page).unwrap().contains(&before_page) {
                        valid = false;
                        rearrange.remove(j);
                        rearrange.insert(insert_pos, page);
                        break 'outer;
                    } else {
                        valid = true;
                    }
                }
            }
        }
        // println!("Found a valid rearrangement");
        part_two_sum += rearrange[rearrange.len() / 2];
    }

    println!("Part one solution: {}", middle_sum);
    println!("Part two solution: {}", part_two_sum);

    Ok(())
}