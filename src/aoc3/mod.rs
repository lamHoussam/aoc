use std::io::{self, BufRead};
use std::fs::File;

enum Value {
    Symbol(char),
    Digit(char),
    CorruptedDigit(char),
}

fn check_is_symbol(c: char) -> bool { !c.is_numeric() && c != '.' }

fn check_digit_with_symbol_nearby(ind_x: usize, ind_y: usize, grid: &Vec<String>) -> Value {
    let line_chars: Vec<(usize, char)> = grid[ind_x].char_indices().collect();
    let value = line_chars.get(ind_y).unwrap();

    if !value.1.is_digit(10) { return Value::Symbol(value.1); }

    if let Some(b) = line_chars.get(ind_y + 1) {
        if check_is_symbol(b.1) { return Value::CorruptedDigit(value.1) }
    }
    

    if ind_y >= 1 {
        let b = line_chars[ind_y - 1];
        if check_is_symbol(b.1) { return Value::CorruptedDigit(value.1) }
    }

    if ind_x + 1 < grid.len() {
        let below_line_chars: Vec<(usize, char)> = grid[ind_x + 1].char_indices().collect();
        if let Some(b) = below_line_chars.get(ind_y) {
            if check_is_symbol(b.1) { return Value::CorruptedDigit(value.1) }
        }
        if let Some(b) = below_line_chars.get(ind_y + 1) {
            if check_is_symbol(b.1) { return Value::CorruptedDigit(value.1) }
        }
        if ind_y >= 1 {
            let b = below_line_chars[ind_y - 1];
            if check_is_symbol(b.1) { return Value::CorruptedDigit(value.1) }
        }
    }
    
    if ind_x >= 1 {
        let above_line_chars: Vec<(usize, char)> = grid[ind_x - 1].char_indices().collect();
        if let Some(b) = above_line_chars.get(ind_y) {
            if check_is_symbol(b.1) { return Value::CorruptedDigit(value.1) }
        }
        if let Some(b) = above_line_chars.get(ind_y + 1) {
            if check_is_symbol(b.1) { return Value::CorruptedDigit(value.1) }
        }
        if ind_y >= 1 {
            let b = above_line_chars[ind_y - 1];
            if check_is_symbol(b.1) { return Value::CorruptedDigit(value.1) }
        }
    }

    return Value::Digit(value.1);
}

pub fn get_engine_sum(file_name: &str) -> Option<i32> {
    let file = File::open(file_name);
    let mut res = 0;

    match file {
        Ok(f) => {

            let reader = io::BufReader::new(f);
            let grid: Vec<String> = reader.lines()
                .map(|content| content.unwrap_or_default())
                .collect();

            let mut num: String = String::new();
            for ind_x in 0..grid.len() {
                let mut is_corrupted = false;
                for ind_y in 0..grid[ind_x].len() {
                    let v = check_digit_with_symbol_nearby(ind_x, ind_y, &grid);
                    match v {
                        Value::Symbol(_) => {
                            if is_corrupted {
                                let v = num.parse::<i32>().unwrap_or_default();
                                res += v;
                            }
                            num.clear();
                            is_corrupted = false;
                        },
                        Value::Digit(d) => {
                            num.push(d);
                        },
                        Value::CorruptedDigit(d) => {
                            num.push(d);
                            is_corrupted = true;
                        }
                    }
                }
            } 

            return Some(res);
        },
        Err(_) => {
            println!("Couldnt open file {}", file_name);
            return None;
        },
    };

}