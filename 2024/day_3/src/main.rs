
use std::fs;
use anyhow::Result;

fn chars_to_u32(chars: &[char]) -> Option<u32> {
    let mut result = 0;

    for &c in chars {
        // Check if the character is a valid digit
        if let Some(digit) = c.to_digit(10) {
            result = result * 10 + digit;
        } else {
            // If the character is not a valid digit, return None
            return None;
        }
    }

    Some(result)
}

fn main() -> Result<()> {
    let corrupted_data = fs::read_to_string("./src/input.txt")?;
    let corrupted_chars = corrupted_data.chars();
    let mut buffer: Vec<char> = vec![];
    let mut first_number = 0;
    let mut second_number = 0;
    let mut comma = 0;
    let mut answer = 0;
    let mut answer2 = 0;
    let mut dont = false;
    for c in corrupted_chars {
        if c == 'd' && buffer.len() == 0 {
            buffer.push(c);
        } else if c == 'o' && buffer.len() == 1 {
            buffer.push(c);
        } else if c == '(' && buffer.len() == 2 && buffer[1] == 'o' {
            buffer.push(c);
        } else if c == ')' && buffer.len() == 3 && buffer[2] == '(' {
            dont = false;
            buffer = vec![];
        } else if c == 'n' && buffer.len() == 2 {
            buffer.push(c);
        } else if c == '\'' && buffer.len() == 3 && buffer[2] == 'n' {
            buffer.push(c);
        } else if c == 't' && buffer.len() == 4 {
            buffer.push(c);
        } else if c == '(' && buffer.len() == 5 {
            buffer.push(c);
        } else if c == ')' && buffer.len() == 6 {
            dont = true;
            buffer = vec![];
        } else {
            if c == 'm' && buffer.len() == 0 {
                buffer.push(c);
            } else if c == 'u' && buffer.len() == 1 {
                buffer.push(c);
            } else if c == 'l' && buffer.len() == 2 {
                buffer.push(c);
            } else if c == '(' && buffer.len() == 3 {
                buffer.push(c);
            }
            // Always first digit
            else if buffer.len() == 4 {
                if c.is_numeric() { buffer.push(c); }
                else { buffer = vec![]; comma = 0; }
            } 
            // Second digit or a comma
            else if buffer.len() == 5 {
                if c.is_numeric() { buffer.push(c); }
                else if c == ',' {
                    buffer.push(c);
                    comma = 5;
                    first_number = chars_to_u32(&buffer[4..comma]).unwrap();
                } else {
                    buffer = vec![];
                    comma = 0;
                }
            }
            // Third digit, a comma, or start of second number
            else if buffer.len() == 6 {
                if c.is_numeric() { buffer.push(c); }
                else if c == ',' && comma == 0 {
                    buffer.push(c);
                    comma = 6;
                    first_number = chars_to_u32(&buffer[4..comma]).unwrap();
                } else {
                    buffer = vec![];
                    comma = 0;
                }
            }
            // Comma, 2nd number 1st digit, 2nd digit
            // or close parentheses
            else if buffer.len() == 7 {
                if c.is_numeric() { buffer.push(c); }
                else if c == ',' && comma == 0 {
                    comma = 7;
                    buffer.push(c);
                    first_number = chars_to_u32(&buffer[4..comma]).unwrap();
                } else if c == ')' && comma !=0 {
                    second_number = chars_to_u32(&buffer[comma+1..]).unwrap();
                    if !dont { answer2 += first_number * second_number; }
                    answer += first_number * second_number;
                    buffer = vec![];
                    comma = 0;
                } else {
                    buffer = vec![];
                    comma = 0;
                }
            }
            // 2nd number 1st digit, 2nd digit, 3rd digit
            // or close parentheses
            else if buffer.len() == 8 {
                if c.is_numeric() { buffer.push(c); }
                else if c == ')' && comma !=0 {
                    second_number = chars_to_u32(&buffer[comma+1..]).unwrap();
                    if !dont { answer2 += first_number * second_number; }
                    answer += first_number * second_number;
                    buffer = vec![];
                    comma = 0;
                }
                else {
                    buffer = vec![];
                    comma = 0;
                }
            }
            // 2nd number 2nd digit, 3rd digit
            // or close parentheses
            else if buffer.len() == 9 {
                if c.is_numeric() { buffer.push(c); }
                else if c == ')' {
                    second_number = chars_to_u32(&buffer[comma+1..]).unwrap();
                    if !dont { answer2 += first_number * second_number; }
                    answer += first_number * second_number;
                    buffer = vec![];
                    comma = 0;
                }
                else {
                    buffer = vec![];
                    comma = 0;
                }
            }
            // 2nd number 3rd digit or close parentheses
            else if buffer.len() == 10 {
                if c.is_numeric() { buffer.push(c); }
                else if c == ')' {
                    second_number = chars_to_u32(&buffer[comma+1..]).unwrap();
                    if !dont { answer2 += first_number * second_number; }
                    answer += first_number * second_number;
                    buffer = vec![];
                    comma = 0;
                }
                else {
                    buffer = vec![];
                    comma = 0;
                }
            }
            // Only close parentheses
            else if buffer.len() == 11 {
                if c == ')' {
                    second_number = chars_to_u32(&buffer[comma+1..]).unwrap();
                    if !dont { answer2 += first_number * second_number; }
                    answer += first_number * second_number;
                    buffer = vec![];
                    comma = 0;
                }
                else {
                    buffer = vec![];
                    comma = 0;
                }
            }  
            else {
                buffer = vec![];
                comma = 0;
            }
        }
    }
    println!("Part one solution: {}", answer);
    println!("Part two solution: {}", answer2);
    Ok(())
}