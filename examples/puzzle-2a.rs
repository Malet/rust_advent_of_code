extern crate rust_advent_of_code;
use rust_advent_of_code::gravity_assist_intcode::{execute, parse};
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
  let lines = io::BufReader::new(File::open("./puzzle-2.txt").expect("Could not open file"))
    .lines()
    .filter_map(Result::ok);

  println!(
    "{}",
    lines
      .map(|l| {
        let mut program = parse(&l);
        program[1] = 12;
        program[2] = 2;
        execute(program)
          .into_iter()
          .map(|i| i.to_string())
          .collect::<Vec<String>>()
          .join(",")
      })
      .collect::<Vec<String>>()
      .join("\n")
  );
}
