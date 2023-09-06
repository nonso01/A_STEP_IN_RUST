#![allow(dead_code)]

// enums vs structs, learning about enums and special Option<T>
//#[derive(Debug)]

// Rust is extremly strict, using  /**/ will
// result as an error, if not used properly.

fn main() {
    let x: i8 = 20; // this is Shadowing
    let x:i8 = x * 2; 

    println!("{x}");
}

