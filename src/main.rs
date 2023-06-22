#![allow(dead_code)]

// enums vs structs, learning about enums and special Option
#[derive(Debug)]

enum Javascript {
    TypeScript(bool, String),
    React(bool),
    Vue(bool),
    Svelte(bool),
    Next(bool),
    Node(bool, String)
}

const JS_TYPES: Option<Javascript> = Some(Javascript::React(false));

fn main() {
    assert_eq!(JS_TYPES.is_none(), false);
}
