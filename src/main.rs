#![allow(dead_code)]
use a_step_in_rust::learn_rust::math_with_int;

fn main() {
    let possible_int = math_with_int(10);
    match possible_int {
        Some(number) => println!("{number}"),
        None => println!("Math Error"),
    }

    assert_eq!(possible_int, Some(3));
}
