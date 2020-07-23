extern crate rust_advent_of_code;
use rust_advent_of_code::wire_intersection::shortest_overall_wire_intersection;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
  let lines: Vec<String> =
    io::BufReader::new(File::open("./puzzle-3.txt").expect("Could not open file"))
      .lines()
      .filter_map(Result::ok)
      .collect();

  println!(
    "{}",
    match shortest_overall_wire_intersection(&lines[0], &lines[1]) {
      Some(distance) => distance.to_string(),
      None => String::from("No intersection"),
    }
  );
}
