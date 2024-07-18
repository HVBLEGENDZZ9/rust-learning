use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read user input from terminal");

    println!("You guessed: {}", guess);
    println!("The secret number was {}", secret_number)
}
