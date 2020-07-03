#[cfg(test)]
mod tests {
  use super::{calc_fuel, calc_total_fuel};
  macro_rules! calc_fuel_tests {
          ($($name:ident: $value:expr,)*) => {
          $(
              #[test]
              fn $name() {
                  let (input, expected) = $value;
                  assert_eq!(expected, calc_fuel(input));
              }
          )*
          }
      }

  macro_rules! calc_total_fuel_tests {
          ($($name:ident: $value:expr,)*) => {
          $(
              #[test]
              fn $name() {
                  let (input, expected) = $value;
                  assert_eq!(expected, calc_total_fuel(input));
              }
          )*
          }
      }

  calc_fuel_tests! {
      calc_fuel_example_12: (12, 2),
      calc_fuel_example_14: (14, 2),
      calc_fuel_example_1969: (1969, 654),
      calc_fuel_example_100756: (100756, 33583),
  }

  calc_total_fuel_tests! {
      calc_total_fuel_example_14: (14, 2),
      calc_total_fuel_example_1969: (1969, 966),
      calc_total_fuel_example_100756: (100756, 50346),
  }
}

fn calc_fuel_recursive(module_mass: i64) -> i64 {
  let fuel_weight = calc_fuel(module_mass);

  if fuel_weight < 0 {
    return 0;
  }

  fuel_weight + calc_fuel_recursive(fuel_weight)
}

pub fn calc_total_fuel(module_mass: i64) -> i64 {
  calc_fuel_recursive(module_mass)
}

pub fn calc_fuel(module_mass: i64) -> i64 {
  (module_mass / 3) - 2
}
