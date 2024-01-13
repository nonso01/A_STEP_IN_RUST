#![allow(dead_code)]
use::std::{collections::HashMap, cmp::Ordering, io};
use rand::Rng;
// Rust is extremly strict, using  /**/ will
// result as an error, if not used properly.

const RUST: &str = "this is a constant";
const BINARY_NUMBER: i8 = 0b110110;

fn main() {
  //  let mut float_x: f32 = 5.5;

  //  show_shadowing();

   // show_mutation(&mut float_x); // mutable reference

  //  show_tuple((35, 40, 70)); // a tuple

  //  fizzbuzz(20);

  //  show_hashmap("hello my friend hello friend");

    // println!("{BINARY_NUMBER}");
    
    // println!("{}", t(Some(0)));
    //
    guessing_game();
}

fn show_shadowing() {
    let x: i8 = 20; // x is an immutable variable
    let x:i8 = x * 2;

    println!("the value of x is {x}");
}

fn show_mutation(old_val: &mut f32) {
    *old_val = 1.5;

    println!("old value has been changed to {old_val}");
}

fn show_tuple(tup: (u8,u8,u8)) { 
    let index_at_zero = tup.0; 
    // indexing a tuple element that does not exist will cause the program to panic

    println!("element at index 0 is {index_at_zero}");
}

fn fizzbuzz(x: u128) {
    for n in 0..x {
        if n % 2 == 0 {
            println!("fizz = {n}");
        } else {
            println!("buzz = {n}");
        }
    }
}

fn show_hashmap(word: &str) {
    let mut map = HashMap::new();

    for w in word.split_whitespace() {
        let count = map.entry(w).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn show_loop() {
    let mut stop_loop: i8 = 0;
loop {
         stop_loop += 1;
          if stop_loop >= 120 {
             println!("stop_loop is {stop_loop}");
             break;
          }
          println!("again");    
    }
}


fn guessing_game() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you Win!!");
                break;
            }
        }
    }
}
