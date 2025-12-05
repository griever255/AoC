use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::fs::File;
use anyhow::Result;

fn open_file() -> io::Result<File> {
    let f = File::open(Path::new("./src/input.txt"))?;
    Ok(f)
}

fn get_adjacents(row: i32, col: i32, grid: &Vec<Vec<char>>) -> Vec<char> {
    let mut adj = vec![];
    for i in -1..=1 {
        for j in -1..=1 {
            match (row + i, col + j) {
                (r, c) if r == row && c == col => continue,
                (r, c) if r < 0 || c < 0 
                    || r as usize >= grid.len() || c as usize >= grid[0].len() => {
                        continue;
                    }, 
                (r, c) => adj.push(grid[r as usize][c as usize]),
            }
        }
    }
    adj
}

fn get_toilet_paper(adj: &Vec<char>) -> u8 {
    adj.iter().filter(|c| **c == '@').count() as u8
}

fn get_position(row: usize, col: usize, grid: &Vec<Vec<char>>) -> char {
    grid[row][col]
}

fn main() -> Result<()> {
    let reader = BufReader::new(open_file().unwrap());
    let mut grid: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line.chars().collect::<Vec<char>>();
        grid.push(row);
    }

    let mut first_answer = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if get_position(row, col, &grid) == '@' && get_toilet_paper(&get_adjacents(row.try_into().unwrap(), col.try_into().unwrap(), &grid)) < 4 {
                first_answer += 1;
            }
        }
    }

    let mut second_answer = 0;
    let mut remove_these: Vec<(usize, usize)> = vec![];
    while true {
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if get_position(row, col, &grid) == '@' && get_toilet_paper(&get_adjacents(row.try_into().unwrap(), col.try_into().unwrap(), &grid)) < 4 {
                    remove_these.push((row, col));
                    second_answer += 1;
                }
            }
        }
        if remove_these.is_empty() {
            break
        }
        for index in &remove_these {
            grid[index.0][index.1] = '.';
        }
        remove_these = vec![];
    }

    // Part One answer
    eprintln!("part 1 solution is: {:?}", first_answer);

    // Part Two answer
    eprintln!("part_2 solution is: {:?}", second_answer);
    Ok(())
}