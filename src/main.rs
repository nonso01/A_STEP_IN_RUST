#![allow(dead_code)]
// use a_step_in_rust::learn_rust::add_binary;
use std::fs::{self, File};

fn main() {
    let a = fs::read_to_string("hello.txt").unwrap();

    println!("{:?}", a);
}
