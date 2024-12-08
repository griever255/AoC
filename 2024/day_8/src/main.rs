use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::fs::File;
use anyhow::Result;
use std::collections::HashMap;

const MAX_ROWS: i32 = 50;
const MAX_COLUMNS: i32 = 50;

fn open_file() -> io::Result<File> {
    let f = File::open(Path::new("./src/input.txt"))?;
    Ok(f)
}


fn main() -> Result<()> {
    let reader = BufReader::new(open_file().unwrap());

    let mut map: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        let line = line?;
        map.push(line.chars().collect::<Vec<char>>());
    }

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for row in 0..MAX_ROWS {
        for col in 0..MAX_COLUMNS {
            let key= map[row as usize][col as usize];
            if key != '.' {
                let locations = antennas.entry(key).or_insert_with(|| vec![]);
                locations.push((row, col));
            }
        }
    }

    let mut anti_nodes: Vec<Vec<char>> = vec![vec!['.'; MAX_COLUMNS as usize]; MAX_ROWS as usize];
    // println!("{:?}", anti_nodes);
    for _antenna in &antennas {
        for locations in antennas.values() {
            // println!("{:?}", locations);
            for i in 0..locations.len() {
                if locations.len() != 1 {
                    anti_nodes[locations[i].0 as usize][locations[i].1 as usize] = '#';
                }
                for j in 0..locations.len() {
                    if i == j {
                        continue;
                    }
                    let distance = (locations[j].0 - locations[i].0, locations[j].1 - locations[i].1);
                    // println!("Distance from {:?} to {:?}: {:?}", locations[i], locations[j], distance);
                    let mut harmonic = 1;
                    loop {
                        let row = locations[j].0 + distance.0 * harmonic;
                        let column = locations[j].1 + distance.1 * harmonic;
                        if row < 0 || row >= MAX_ROWS {
                            break
                        }
                        if column < 0 || column >= MAX_COLUMNS{
                            break;
                        }
                        // println!("Antinode location: ({:?} {:?})", row, column);
                        anti_nodes[row as usize][column as usize] = '#';
                        harmonic += 1;
                    }
                }
            }
        }
    }

    let mut num_anti_nodes = 0;
    for row in 0..MAX_ROWS {
        for col in 0..MAX_COLUMNS {
            let key= anti_nodes[row as usize][col as usize];
            if key == '#' {
                num_anti_nodes += 1;
            }
        }
    }

    // println!("{:?}", anti_nodes);
    // println!("{:?}", map);
    // println!("{:?}", antennas);
    println!("Part two solution: {:?}", num_anti_nodes);

    Ok(())


}