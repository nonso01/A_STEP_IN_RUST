#![allow(dead_code)]

//#[derive(Debug)]

// Rust is extremly strict, using  /**/ will
// result as an error, if not used properly.

const RUST: &str = "this is a constant";

fn main() {
    show_shadowing();
}

fn show_shadowing() {
    let x: i8 = 20; // x is an immutable variable
    let x:i8 = x * 2;

    println!("the value of x is {x}");
}

fn show_mutation() {
    let mut y = ;
    y = 1_000;

    println!("y has been changed to {y}");
}
