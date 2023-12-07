use regex::Regex;
use std::fs::File;

use std::io::{self, BufRead};
const MAX_RED  : i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE : i32 = 14;

fn check_number_cubes(number: i32, color: &str) -> bool {
    if number == -1 {
        return false;
    } 

    return match color {
        "red" => number <= MAX_RED,
        "blue" => number <= MAX_BLUE,
        "green" => number <= MAX_GREEN,
        _ => false,
    }
}

pub fn get_correct_ids_sum(file_name: &str) -> Option<i32> {
    let file = File::open(file_name);

    return match file {
        Ok(f) => {
            let reader = io::BufReader::new(f);
            Some(reader.lines()
                .map(|content| parse_line(content.unwrap_or_default()))
                .into_iter().sum()) 
        },
        Err(_) => {
            println!("Couldnt open file {}", file_name);
            return None;
        },
    };
}

fn parse_line(msg: String) -> i32 {
    let pattern = r"Game (?P<id>\d+): (?P<sequence>(\d+ (red|blue|green)(,|;) )*\d+ (red|blue|green))";
    let regexp = Regex::new(pattern).unwrap();

    let mut game_id = 0;

    if let Some(captures) = regexp.captures(msg.as_str()) {
        game_id = captures.name("id")
            .unwrap().as_str().parse::<i32>()
            .unwrap_or_default();

        if let Some(sequence) = captures.name("sequence") {
            let values_colors: Vec<(&str, &str)> = sequence.as_str()
                .split(|c| c == ',' || c == ';').map(|s| {
                    let mut parts = s.trim().splitn(2, ' ');
                    (parts.next().unwrap(), parts.next().unwrap())
                }).collect();
            
            for (v, c) in values_colors {
                if !check_number_cubes(v.parse::<i32>().unwrap_or(-1), c) {
                    return 0
                }
            }
        }
    }
    return game_id;

}


