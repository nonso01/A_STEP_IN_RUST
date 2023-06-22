#![allow(dead_code)]

// enums vs structs, learning about enums and special Option
#[derive(Debug)]
enum Kingdom<'a> { // <'a> is knowns as a lifetime, will cover it soon
    Monera(String),
    Protoctista(String),
    Fungi(String),
    Plantae(String),
    Animalia(&'a str),
}

const SOME_INT: Option<i8> = Some(50);
// static here is for global var, might use const too
static NO_INT: Option<i8> = None;

fn main() {
    let human = Kingdom::Animalia("mammals");
    println!("human is {:?}", human);

    assert_eq!(SOME_INT.is_some(), true);
    assert_eq!(NO_INT.is_none(), true);
}
