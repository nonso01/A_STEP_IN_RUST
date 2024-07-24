#![allow(dead_code)]
use a_step_in_rust::learn_rust::frequency_sort;

fn main() {
    let array = vec![2, 2, 2, 2, 1, 1, 1, 3, 3, 4, 4, -1, 5];
    let _sorted = frequency_sort(&array);

    println!("{:?}", _sorted);
}
