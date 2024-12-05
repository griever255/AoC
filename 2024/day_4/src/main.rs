use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::fs::File;
use anyhow::Result;

const MAX_ROWS: usize = 140;
const MAX_COLUMNS: usize = 140;

fn open_file() -> io::Result<File> {
    let f = File::open(Path::new("./src/input.txt"))?;
    Ok(f)
}

fn continue_in_direction(crossword: &Vec<Vec<char>>, row: usize, col: usize, delta_row: i32, delta_col: i32) -> Option<char> {
    let search_row = row as i32 + delta_row;
    if search_row < 0 {
        return None
    }
    let search_col = col as i32 + delta_col;
    if search_col < 0 {
        return None
    }
    let search_row = search_row as usize;
    let search_col = search_col as usize;
    if search_row < MAX_ROWS && search_col < MAX_COLUMNS {
        Some(crossword[search_row][search_col])
    } else { None }
}

fn get_adjacents(crossword: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<Option<(char, i32, i32)>> {
    // println!("Getting adjacents for row: {} and col: {}", row, col);
    let top_left = if row != 0 && col != 0 {
        Some((crossword[row-1][col-1], -1, -1))
    } else { None };
    let top_mid = if row != 0 {
        Some((crossword[row-1][col], -1, 0))
    } else { None };
    let top_right = if row != 0 && col != MAX_COLUMNS-1 {
        Some((crossword[row-1][col+1], -1, 1))
    } else { None };
    let left = if col != 0 {
        Some((crossword[row][col-1], 0, -1))
    } else { None };
    let right = if col != MAX_COLUMNS-1 {
        Some((crossword[row][col+1], 0, 1))
    } else { None };
    let bot_left = if row != MAX_ROWS-1 && col != 0 {
        Some((crossword[row+1][col-1], 1, -1))
    } else { None };
    let bot_mid = if row != MAX_ROWS-1 {
        Some((crossword[row+1][col], 1, 0))
    } else { None };
    let bot_right = if row != MAX_ROWS-1 && col != MAX_COLUMNS-1 {
        Some((crossword[row+1][col+1], 1, 1))
    } else { None };
    vec![
        top_left,
        top_mid,
        top_right,
        left,
        right,
        bot_left,
        bot_mid,
        bot_right,
    ]
}

fn main() -> Result<()> {
    let reader = BufReader::new(open_file().unwrap());

    let mut crossword: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        let line = line?;
        crossword.push(line.chars().collect::<Vec<char>>());
    }

    let mut num_xmas = 0;
    let mut part_two = 0;
    for row in 0..MAX_ROWS {
        for col in 0..MAX_COLUMNS {
            if crossword[row][col] == 'X' {
                for m in get_adjacents(&crossword, row, col) {
                    if let Some(m) = m {
                        if m.0 == 'M' {
                            if continue_in_direction(&crossword, row, col, m.1 * 2, m.2 * 2) == Some('A') {
                                if continue_in_direction(&crossword, row, col, m.1 * 3, m.2 * 3) == Some('S') {
                                    num_xmas += 1;
                                }
                            }
                        }
                    }
                }
            }
            if crossword[row][col] == 'A' {
                let letters = get_adjacents(&crossword, row, col);
                if letters[0].is_some() && letters[2].is_some() && letters[5].is_some() && letters[7].is_some() {
                    if letters[0].unwrap().0 == 'M' && letters[2].unwrap().0 == 'M' && letters[5].unwrap().0 == 'S' && letters[7].unwrap().0 == 'S' {
                        part_two += 1;
                    } else if letters[0].unwrap().0 == 'M' && letters[2].unwrap().0 == 'S' && letters[5].unwrap().0 == 'M' && letters[7].unwrap().0 == 'S' {
                        part_two += 1;
                    } else if letters[0].unwrap().0 == 'S' && letters[2].unwrap().0 == 'S' && letters[5].unwrap().0 == 'M' && letters[7].unwrap().0 == 'M' {
                        part_two += 1;
                    } else if letters[0].unwrap().0 == 'S' && letters[2].unwrap().0 == 'M' && letters[5].unwrap().0 == 'S' && letters[7].unwrap().0 == 'M' {
                        part_two += 1;
                    }
                }
            }
        }
    }

    println!("Part one answer: {}", num_xmas);
    println!("Part two answer: {}", part_two);
    Ok(())
}