#![allow(dead_code)]
use a_step_in_rust::learn_rust::add_binary;

fn main() {
    let _test = add_binary("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string(), "100".to_string());

    println!("{_test}");
}

fn b_to_d(binary_str: &str) -> Result<i128, String> {
    let mut decimal = 0;
    let mut base = 1;

    for digit in binary_str.chars().rev() {
        if digit != '0' && digit != '1' {
            return Err("invalid binary string".to_string());
        }

        decimal += (digit as i128 - '0' as i128) * base;
        base *= 2;
    }

    Ok(decimal)
}
