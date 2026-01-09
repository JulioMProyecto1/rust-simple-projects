//1. CLI Number Guessing Game
//A terminal game where the program picks a random number and the user tries to guess it.
use rand::Rng;
use std::cmp;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io;

fn main() {
    // Set random number from 1 to 100
    let secret_number = rand::rng().random_range(1..=100);
    loop {
        // Sets input from user
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error reading line from user");
        // Compare two values and give back result if it's the same

        let guess: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Less => {
                println!("Too small!")
            }
            Equal => {
                println!("You won!");
                break;
            }
            Greater => {
                println!("Too big!")
            }
        }
    }
}
