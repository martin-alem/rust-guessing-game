use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Stdin};

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter a number:");

        let mut guess: String = String::new();

        let input_handle: Stdin = io::stdin();

        input_handle
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Guess too high"),
            Ordering::Less => println!("Guess too low"),
            Ordering::Equal => {
                println!("Congratulations, you guessed the secret number");
                break;
            }
        }
    }
}
