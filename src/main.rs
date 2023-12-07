use crate::aoc1::*;
use crate::aoc2::*;

mod aoc2;
mod aoc1;


fn main() {
    // let v1 = get_code_sum("src/aoc1/input.txt"); 
    // println!("AOC1: {}", v1.unwrap_or(-1));

    // let v2 = get_code_sum2("src/aoc1/input.txt"); 
    // println!("AOC2: {}", v2.unwrap_or(-1));

    let v = get_correct_ids_sum("src/aoc2/input.txt");
    println!("{}", v.unwrap_or_default());
}
