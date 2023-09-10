#![allow(dead_code)]
// enums vs structs, learning about enums and special Option<T>

//#[derive(Debug)]

// Rust is extremly strict, using  /**/ will
// result as an error, if not used properly.

const RUST: &str = "this is a constant"; // string slice

fn main() {
    show_shadowing();
}

fn show_shadowing() {
    let x: i8 = 20; // x is an immutable variable
    let x:i8 = x * 2;

    println!("the value of x is {x}");
}

fn show_mutation() {
    let mut y: i16 = 0;
    y = 1_000;

    println!("y has been changed to {y}");
}

fn show_tuple(&tup) {

}
