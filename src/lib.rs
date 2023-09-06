#![allow(dead_code)]
// Hello!,  here are brief notes of what i learn in
// ~~~~~~~~~~~~ RUST ~~~~~~~~~~~~


// since we are in a library crate, we are
// going to use `const` do declare global vars.

pub mod what_is_rust {
    pub fn rust_is() -> &'static str {
        // haven't covered lifetime yet
        "Rust is a statically typed language, like C, but sometimes knows what you mean out of the box. Widely used for the creation of a large variety of softwares, ranging from servers , database, embeded sys and AI."
    }
}
