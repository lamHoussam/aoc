use std::fs::File;
use std::io::{self, BufRead};

fn get_code_digits(msg: String) -> i32 {
    let values: Vec<char> =  msg.chars().filter(|m| m.is_digit(10)).collect();
    let value = if values.len() != 0 {
        values[0].to_string() + &values[values.len() - 1].to_string()
    } else {
        "0".to_string()
    };

    return value.parse::<i32>().unwrap_or_default();
}

pub fn get_code_sum(file_name: &str) -> Option<i32> {
    let file = File::open(file_name);

    return match file {
        Ok(f) => {
            let reader = io::BufReader::new(f);
            Some(reader.lines()
                .map(|content| get_code_digits(content.unwrap_or_default()))
                .into_iter().sum()) 
        },
        Err(_) => {
            println!("Couldnt open file {}", file_name);
            return None;
        },
    }; 
}