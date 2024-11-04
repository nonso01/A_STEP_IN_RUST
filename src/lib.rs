#![allow(dead_code)]

pub mod learn_rust {
    use ::std::{
        cmp::Ordering,
        collections::HashMap,
        fs::File,
        io::{self, Read},
    };
    use rand::Rng;

    pub fn show_shadowing() {
        let x: i8 = 20; // x is an immutable variable
        let x: i8 = x * 2;

        println!("the value of x is {x}");
    }

    pub fn show_mutation(old_val: &mut f32) {
        *old_val = 1.5;

        println!("old value has been changed to {old_val}");
    }

    pub fn show_tuple(tup: (u8, u8, u8)) {
        let index_at_zero = tup.0;
        // indexing a tuple element that does not exist will cause the program to panic
        println!("element at index 0 is {index_at_zero}");
    }

    pub fn fizzbuzz(x: u128) {
        for n in 0..x {
            if n % 2 == 0 {
                println!("fizz = {n}");
            } else {
                println!("buzz = {n}");
            }
        }
    }

    pub fn show_hashmap(word: &str) {
        let mut map = HashMap::new();

        for w in word.split_whitespace() {
            let count = map.entry(w).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    }

    pub fn show_loop() {
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

    pub fn guessing_game() {
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
                Err(_) => continue,
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

    pub fn quadratic_formula(a: f32, b: f32, c: f32) -> (f32, f32) {
        // x = (-b ± √b2 - 4ac) / 2a
        let value_in_square_root: f32 = (b * b) - 4.0 * a * c;
        let root1: f32 = (-b + value_in_square_root.sqrt()) / (2.0 * a);
        let root2: f32 = (-b - value_in_square_root.sqrt()) / (2.0 * a);

        (root1, root2)
    }

    pub fn read_file_content(filename: &str) -> Result<String, io::Error> {
        let mut _file = File::open(filename)?;
        let mut file_content = String::new();

        _file.read_to_string(&mut file_content)?;
        Ok(file_content)
    }

    pub fn frequency_sort(n: &Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = n.to_vec();
        let mut freq_map = HashMap::new();

        for number in res.clone().into_iter() {
            *freq_map.entry(number).or_insert(0) += 1
        }

        res.sort_by(|a, b| {
            let freq_a = freq_map.get(a).unwrap();
            let freq_b = freq_map.get(b).unwrap();

            if freq_a == freq_b {
                b.cmp(a)
            } else {
                freq_a.cmp(&freq_b)
            }
        });
        res
    }

    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        // try iter::successors, pascal 1
        let mut res: Vec<Vec<i32>> = Vec::with_capacity(num_rows as usize);

        if num_rows == 0 {
            return res;
        }
        res.push(vec![1]);

        for i in 1..num_rows {
            let prev = &res[i as usize - 1];
            let mut next = vec![1];

            for j in 1..res.len() {
                next.push(&prev[j - 1] + &prev[j]);
            }
            next.push(1);
            res.push(next);
        }

        res
    }

    pub fn add_binary(_a: String, _b: String) -> String {
        "hello".to_string()
    }
}
