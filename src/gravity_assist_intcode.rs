#[cfg(test)]
mod tests {
  use super::{parse, run};

  #[test]
  #[should_panic(expected = "Could not parse program: (a): ParseIntError { kind: InvalidDigit }")]
  fn parse_panics_on_invalid_input() {
    parse("a");
  }

  #[test]
  fn parse_returns_program_as_vec_i64() {
    let expected: Vec<i64> = vec![1, 2, 3];
    assert_eq!(expected, parse("1,2,3"));
  }

  #[test]
  fn run_supports_add_operator() {
    assert_eq!("2,0,0,0,99", run("1,0,0,0,99"))
  }

  #[test]
  fn run_supports_end_operator() {
    assert_eq!("30,1,1,4,2,5,6,0,99", run("1,1,1,4,99,5,6,0,99"))
  }

  #[test]
  fn run_supports_multiply_operator() {
    assert_eq!("2,3,0,6,99", run("2,3,0,3,99"))
  }

  #[test]
  fn run_supports_multiply_operator_with_end() {
    assert_eq!("2,4,4,5,99,9801", run("2,4,4,5,99,0"))
  }
}
pub fn run(program: &str) -> String {
  execute(parse(program))
    .into_iter()
    .map(|i| i.to_string())
    .collect::<Vec<String>>()
    .join(",")
}

pub fn execute(mut program: Vec<i64>) -> Vec<i64> {
  let mut offset = 0;
  loop {
    let operator = program[0 + offset];
    if operator == 99 {
      break;
    }
    let input1_location = program[1 + offset] as usize;
    let input2_location = program[2 + offset] as usize;
    let output_location = program[3 + offset] as usize;
    match operator {
      1 => program[output_location] = program[input1_location] + program[input2_location],
      2 => program[output_location] = program[input1_location] * program[input2_location],
      _ => panic!("Unrecognised operator"),
    }
    offset += 4;
  }

  program
}

pub fn parse(program: &str) -> Vec<i64> {
  program
    .split(',')
    .map(|l| {
      l.parse::<i64>()
        .expect(&format!("Could not parse program: ({})", program))
    })
    .collect()
}
