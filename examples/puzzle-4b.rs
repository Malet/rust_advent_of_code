extern crate rust_advent_of_code;
use rust_advent_of_code::secure_container::scan_passwords_2;

fn main() {
    println!("{:?}", scan_passwords_2(130254..=678275).len());
}
