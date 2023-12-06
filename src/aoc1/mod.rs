use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn get_digit_from_str(val: &str) -> String {
    return match val {
        "zero" => "0",
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        f => f,
    }.to_string();
}

fn get_code_digits2(msg: String) -> i32 {

    let patt1 = r#"(\d|zero|one|two|three|four|five|six|seven|eight|nine)"#;
    let patt2 = r#"(\d|orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)"#;

    let reg1 = Regex::new(patt1).unwrap();
    let reg2 = Regex::new(patt2).unwrap();

    let rev_msg: String = msg.chars().rev().collect();

    return if let Some(mat) = reg2.find(&rev_msg) {
        let start = msg.len() - mat.end();
        let end = msg.len() - mat.start();

        let last = &msg[start..end];        
        let frst = reg1.find(&msg).unwrap().as_str();

        let final_value = get_digit_from_str(frst) + &get_digit_from_str(last);
        final_value.parse::<i32>().unwrap_or_default()
    } else { 0 };
}

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

pub fn get_code_sum2(file_name: &str) -> Option<i32> {
   let file = File::open(file_name);

    return match file {
        Ok(f) => {
            let reader = io::BufReader::new(f);
            Some(reader.lines()
                .map(|content| get_code_digits2(content.unwrap_or_default()))
                .into_iter().sum()) 
        },
        Err(_) => {
            println!("Couldnt open file {}", file_name);
            return None;
        },
    }; 
}