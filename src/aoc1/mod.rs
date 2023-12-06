use std::fs::File;
use std::io::{self, BufRead};

pub fn get_code_digits(msg: String) -> i32 {
    let mut frst: Option<char> = None;
    let mut last: Option<char> = None;

    for i in msg.chars() {
        if i.is_digit(10) {
            if frst == None {
                frst = Some(i);
                last = Some(i);
            } else {
                last = Some(i);
            }
        }
    }

    let value = frst.unwrap_or_default().to_string() + &last.unwrap_or_default().to_string(); 
    return value.parse::<i32>().unwrap_or_default();
}

pub fn get_code_sum(file_name: &str) -> Option<i32> {

    let mut res: i32 = 0;
    let file = File::open(file_name);

    match file {
        Ok(f) => {
            let reader = io::BufReader::new(f);

            for line in reader.lines() {
                match line {
                    Ok(content) => {
                        res += get_code_digits(content);
                    },
                    Err(_) => {
                        println!("Couldnt open file {}", file_name);
                        return None;
                    },
                }
            }
        },
        Err(_) => {
            println!("Couldnt open file {}", file_name);
            return None;
        },
    } 


    return Some(res);
}