use std::cmp;
use std::collections::HashSet;
use std::iter::FromIterator;

#[cfg(test)]
mod tests {
  use super::{
    calculate_overlap, calculate_path_length, closest_wire_intersection, generate_wire_path, parse,
    Direction, Instruction,
  };
  use std::collections::HashSet;
  use std::iter::FromIterator;

  #[test]
  #[should_panic(expected = "Cannot parse string [A] to u64: ParseIntError { kind: InvalidDigit }")]
  fn parse_panics_on_invalid_distance() {
    parse("UA,DB");
  }

  #[test]
  #[should_panic(expected = "Unable to parse direction from: [A]")]
  fn parse_panics_on_invalid_direction() {
    parse("A1,B2");
  }

  #[test]
  fn calculate_path_length_from_instructions() {
    assert_eq!(5, calculate_path_length(&parse("U1,D1,R1,L1")));
  }

  #[test]
  fn closest_wire_intersection_from_program() {
    // .+-+..
    // .o.|..
    // .+-X-. (2,-1)
    // ...|..
    assert_eq!(Some(3), closest_wire_intersection("D1,R3", "U1,R2,D3"))
  }

  #[test]
  fn calculate_overlap_from_paths() {
    let expected: HashSet<(i64, i64)> = [(1, 1), (1, 2)].iter().cloned().collect();
    let actual = calculate_overlap(
      vec![(0, 1), (1, 1), (1, 2)],
      vec![(3, 2), (2, 2), (1, 2), (1, 1)],
    );
    assert_eq!(expected, HashSet::from_iter(actual.into_iter()));
  }

  #[test]
  fn parse_instruction() {
    assert_eq!(
      vec![
        Instruction {
          direction: Direction::Up,
          distance: 1
        },
        Instruction {
          direction: Direction::Down,
          distance: 2
        },
        Instruction {
          direction: Direction::Right,
          distance: 3
        },
        Instruction {
          direction: Direction::Left,
          distance: 10
        }
      ],
      parse("U1,D2,R3,L10").as_slice()
    );
  }
  #[test]
  fn generate_wire_path_from_instruction() {
    assert_eq!(
      vec![
        (0, 0),
        (0, 1),
        (0, 0),
        (0, -1),
        (1, -1),
        (2, -1),
        (3, -1),
        (2, -1),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-2, -1),
        (-3, -1),
        (-4, -1),
        (-5, -1),
        (-6, -1),
        (-7, -1)
      ],
      generate_wire_path(parse("U1,D2,R3,L10")).as_slice()
    );
  }
}

#[derive(Debug, PartialEq)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
  direction: Direction,
  distance: u64,
}

fn manhattan_distance(x: i64, y: i64) -> u64 {
  (x.abs() + y.abs()) as u64
}

pub fn closest_wire_intersection(program_a: &str, program_b: &str) -> Option<u64> {
  calculate_overlap(
    generate_wire_path(parse(program_a)),
    generate_wire_path(parse(program_b)),
  )
  .into_iter()
  .map(|(x, y)| manhattan_distance(x, y))
  .fold(None, |acc, distance_a| match acc {
    Some(distance_b) => Some(cmp::min(distance_a, distance_b)),
    None => Some(distance_a),
  })
}

pub fn generate_wire_path(instructions: Vec<Instruction>) -> Vec<(i64, i64)> {
  let mut path = Vec::<(i64, i64)>::with_capacity(calculate_path_length(&instructions) as usize);
  path.push((0, 0));
  for instruction in instructions {
    for _i in 1..=instruction.distance {
      let (mut x, mut y) = path
        .last()
        .expect("Could not get last item in path")
        .clone();
      match instruction.direction {
        Direction::Up => y += 1,
        Direction::Down => y -= 1,
        Direction::Left => x -= 1,
        Direction::Right => x += 1,
      };
      path.push((x, y));
    }
  }
  path
}

pub fn calculate_overlap(path_a: Vec<(i64, i64)>, path_b: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
  // Remove origin with `.skip(1)` as it'll always overlap
  let path_a_hash: HashSet<_> = path_a.iter().skip(1).cloned().collect();
  let path_b_hash: HashSet<_> = path_b.iter().skip(1).cloned().collect();
  Vec::from_iter(path_a_hash.intersection(&path_b_hash).cloned())
}

pub fn calculate_path_length(instructions: &Vec<Instruction>) -> u64 {
  instructions.into_iter().fold(1, |acc, i| acc + i.distance)
}

pub fn parse(program: &str) -> Vec<Instruction> {
  let instructions = program.split(",");
  instructions
    .map(|i| {
      let (direction_char, distance_str) = i.split_at(1);
      let distance = distance_str
        .parse::<u64>()
        .expect(format!("Cannot parse string [{}] to u64", distance_str).as_str());

      let direction = match direction_char {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!(format!(
          "Unable to parse direction from: [{}]",
          direction_char
        )),
      };

      Instruction {
        direction,
        distance,
      }
    })
    .collect()
}
