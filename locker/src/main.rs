use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    loop {
        let mut guess = String::new();

        let secret_number = rand::thread_rng().gen_range(1, 101);

        println!("Random number: {}", secret_number);

        println!("Please enter your number!");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Your number is less than the secret number".red()),
            Ordering::Greater => {
                println!("{}", "Your number is greaten than the secret number".red())
            }
            Ordering::Equal => {
                println!("{}", "You guessed the number!".green());
                break;
            }
        }
    }
}
