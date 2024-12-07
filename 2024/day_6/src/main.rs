use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::fs::File;
use anyhow::Result;

const MAX_ROWS: usize = 130;
const MAX_COLUMNS: usize = 130;

#[derive(Clone, PartialEq)]
struct Guard {
    position: (i32, i32),
    orientation: (i32, i32),
}

impl Guard {
    fn rotate_90(&mut self) {
        match self.orientation {
            (-1, 0) => self.orientation = (0, 1),
            (0, 1) => self.orientation = (1, 0),
            (1, 0) => self.orientation = (0, -1),
            (0, -1) => self.orientation = (-1, 0),
            (_, _) => unreachable!(),
        }
    }

    fn step(&mut self) {
        self.position = (self.position.0 + self.orientation.0, self.position.1 + self.orientation.1);
    }
    
    fn reset(&mut self, starting_position: (i32, i32)) {
        self.position = starting_position;
        self.orientation = (-1, 0);
    }
}

fn open_file() -> io::Result<File> {
    let f = File::open(Path::new("./src/input.txt"))?;
    Ok(f)
}

fn is_object_in_front(map: &Vec<Vec<char>>, guard: &Guard) -> bool {
    let row = guard.position.0 + guard.orientation.0;
    let col = guard.position.1 + guard.orientation.1;

    if within_map((row, col)) {
        let row = row as usize;
        let col = col as usize;
        map[row][col] == '#'
    } else {
        false
    }
}

fn within_map(position: (i32, i32)) -> bool {
    position.0 >= 0 && position.0 < MAX_ROWS.try_into().unwrap() && position.1 >= 0 && position.1 < MAX_COLUMNS.try_into().unwrap()
}

fn main() -> Result<()> {
    let reader = BufReader::new(open_file().unwrap());

    let mut map: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        let line = line?;
        map.push(line.chars().collect::<Vec<char>>());
    }
    
    let mut starting_position = (0, 0);
    let mut guard = Guard {
        position: starting_position,
        orientation: (-1, 0),
    };
    for row in 0..MAX_ROWS {
        for col in 0..MAX_COLUMNS {
            if map[row][col] == '^' {
                starting_position = (row as i32, col as i32);
                guard.position = starting_position;
            }
        }
    }

    let mut distinct_positions: Vec<(i32, i32)> = vec![];
    
    while within_map(guard.position) {
        if is_object_in_front(&map, &guard) {
            guard.rotate_90();
        } else {
            if !distinct_positions.contains(&guard.position) {
                distinct_positions.push(guard.position);
            }
            guard.step();
        }
    }

    let mut num_loops = 0;
    for row in 0..MAX_ROWS {
        'col: for col in 0..MAX_COLUMNS {
            guard.reset(starting_position);
            let mut test_map = map.clone();
            let mut trail = vec![];
            if test_map[row][col] == '^' || test_map[row][col] == '#' {
                continue 'col;
            } else {
                test_map[row][col] = '#';
            }
            let mut is_loop = false;
            'inner: while !is_loop {
                if trail.contains(&guard) {
                    num_loops += 1;
                    is_loop = true;
                }

                if is_object_in_front(&test_map, &guard) {
                    guard.rotate_90();
                } else {
                    trail.push(guard.clone());
                    guard.step();
                }

                if !within_map(guard.position) {
                    break 'inner; 
                }
            }
        }
    }

    println!("Distinct Positions: {}", distinct_positions.len());
    println!("Number of loops: {}", num_loops);
    Ok(())
}