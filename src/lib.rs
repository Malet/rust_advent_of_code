pub mod fuel {
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
}

pub mod gravity_assist_intcode {
    #[cfg(test)]
    mod tests {
        use super::{parse, run};

        #[test]
        #[should_panic(
            expected = "Could not parse program: (a): ParseIntError { kind: InvalidDigit }"
        )]
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
}
