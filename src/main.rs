
mod aoc2;
mod aoc1;


fn main() {
/*
    /// Day 1
    let v1 = gcrate::aoc1::et_code_sum("src/aoc1/input.txt"); 
    println!("AOC1: {}", v1.unwrap_or(-1));

    /// Part 2
    let v2 = crate::aoc1::get_code_sum2("src/aoc1/input.txt"); 
    println!("AOC2: {}", v2.unwrap_or(-1));

*/

/*
    ///Day 2
    let v = crate::aoc2::get_correct_ids_sum("src/aoc2/input.txt");
    println!("{}", v.unwrap_or_default());

    /// Part2
    
    */
    
    let v = crate::aoc2::get_power_set_sum("src/aoc2/input.txt");
    println!("{}", v.unwrap_or_default());

}
