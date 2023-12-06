use crate::aoc1::*;

mod aoc1;

fn main() {
    let v1 = get_code_sum("src/aoc1/input.txt"); 
    println!("AOC1: {}", v1.unwrap_or(-1));
}
