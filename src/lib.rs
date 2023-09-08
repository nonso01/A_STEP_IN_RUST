#![allow(dead_code)]


// since we are in a library crate, we are
// going to use `const` to declare global vars.

pub mod what_is_rust {
    pub fn rust_is() -> String {
        "Rust is a statically typed language, like C, but sometimes knows what you mean out of the box. And it's widely used for the creation of a large variety of softwares, ranging from servers , database, embeded sys and AI.".to_string()
    }
}
