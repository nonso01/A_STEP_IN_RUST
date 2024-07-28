#![allow(dead_code)]
use a_step_in_rust::learn_rust::restore_ip_addresses;

fn main() {
    let test_string = String::from("255000111222");
    let res = restore_ip_addresses(&test_string);

    println!("{:?}", res);
}
