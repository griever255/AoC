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
    let mut position: i16 = 50; 
    let mut part_1 = 0;
    let mut part_2 = 0;

    for line in reader.lines() {
        let line = line?;
        let (direction, amount) = line.split_at(1);
        let mut amount = amount.parse::<i16>()?;
        part_2 += amount / 100;
        amount %= 100;
        match direction {
            "L" => {
                position -= amount;
                if position == 0 && amount != 0 {
                    part_2 += 1;
                } else if position < 0 {
                    if position + amount != 0 {
                        part_2 += 1;
                    }
                    position += (-position / 100 + 1) * 100;
                }
            },
            "R" => {
                position += amount;
                if position >= 100 {
                    part_2 += 1;
                }
            },
            _ => unreachable!(),
        }

        if position % 100 == 0 {
            part_1 += 1;
        }

        position %= 100;
    }

    // Part One answer
    eprintln!("part 1 solution is: {:?}", part_1);

    // Part Two answer
    eprintln!("part_2 solution is: {:?}", part_2);
    Ok(())
}