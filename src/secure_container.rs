use std::collections::HashMap;

#[cfg(test)]
mod tests {
  use super::scan_passwords_1;
  #[test]
  fn should_scan_passwords() {
    assert_eq!(vec![11], scan_passwords_1(10..=12));
  }
}

pub fn scan_passwords_1(range: std::ops::RangeInclusive<u64>) -> Vec<u64> {
  range
    .filter(|number| has_adjacent_duplicates(*number) && is_increasing(*number))
    .collect()
}

pub fn scan_passwords_2(range: std::ops::RangeInclusive<u64>) -> Vec<u64> {
  range
    .filter(|number| {
      has_adjacent_duplicates(*number)
        && has_adjacent_duplicates_without_larger_group(*number)
        && is_increasing(*number)
    })
    .collect()
}

#[test]
fn should_detect_has_adjacent_duplicates_11() {
  assert_eq!(true, has_adjacent_duplicates(11));
}
#[test]
fn should_detect_has_adjacent_duplicates_12() {
  assert_eq!(false, has_adjacent_duplicates(12));
}
#[test]
fn should_detect_has_adjacent_duplicates_112() {
  assert_eq!(true, has_adjacent_duplicates(112));
}

#[test]
fn should_detect_has_adjacent_duplicates_122() {
  assert_eq!(true, has_adjacent_duplicates(122));
}

#[test]
fn should_detect_has_adjacent_duplicates_1221() {
  assert_eq!(true, has_adjacent_duplicates(1221));
}

fn is_increasing(number: u64) -> bool {
  let passcode = number.to_string();
  let mut prev = None;
  for digit in passcode.chars() {
    let digit_int = digit.to_digit(10).expect("Could not convert to digit");
    match prev {
      Some(previous_digit) => {
        if digit_int < previous_digit {
          return false;
        }
      }
      None => {}
    }
    prev = Some(digit_int);
  }
  true
}

fn has_adjacent_duplicates(number: u64) -> bool {
  let passcode = number.to_string();
  let mut prev = None;
  for digit in passcode.chars() {
    match prev {
      Some(previous_digit) => {
        if previous_digit == digit {
          return true;
        }
      }
      None => {}
    }
    prev = Some(digit);
  }
  false
}

fn has_adjacent_duplicates_without_larger_group(number: u64) -> bool {
  let passcode = number.to_string();
  let mut buffer: HashMap<char, u8> = HashMap::new();
  let mut prev = None;
  for digit in passcode.chars() {
    match prev {
      Some(previous_digit) => {
        if previous_digit == digit {
          *buffer.entry(digit).or_insert(1) += 1;
        }
      }
      None => {}
    }
    prev = Some(digit);
  }
  buffer.iter().any(|(_, v)| *v == 2)
}
