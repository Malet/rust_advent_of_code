extern crate rust_advent_of_code;
use rust_advent_of_code::gravity_assist_intcode::{execute, parse};
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
  let lines = io::BufReader::new(File::open("./puzzle-2.txt").expect("Could not open file"))
    .lines()
    .filter_map(Result::ok);

  lines.for_each(|l| {
    let program = parse(&l);
    for noun in 1..99 {
      for verb in 1..99 {
        let mut program_copy = program.to_vec();
        program_copy[1] = noun;
        program_copy[2] = verb;
        let result = execute(program_copy);

        if result[0] == 19690720 {
          println!(
            "noun: {}, verb: {}, answer: {}",
            noun,
            verb,
            100 * noun + verb
          );
        }
      }
    }
  })
}
