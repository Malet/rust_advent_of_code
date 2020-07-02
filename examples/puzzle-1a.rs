extern crate rust_advent_of_code;
use rust_advent_of_code::fuel::calc_fuel;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(File::open("./puzzle-1.txt").expect("Could not open file"))
        .lines()
        .filter_map(Result::ok);

    println!(
        "{}",
        lines
            .filter_map(|l| l.parse::<i64>().ok())
            .map(calc_fuel)
            .sum::<i64>()
            .to_string()
    );
}
