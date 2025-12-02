use aoc_2025_rs::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't open file");
    let output = solver_01_2(&&input);

    println!("{output}");
}
