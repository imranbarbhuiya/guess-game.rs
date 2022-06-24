use rand::{thread_rng, Rng};
use std::{cmp::Ordering, io::stdin};

fn main() {
    println!("Welcome to guessing game");

    let secret_number = thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line");

        let input: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", input);

        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You guessed it");
                break;
            }
        }
    }
}
