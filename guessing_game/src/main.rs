use std::io;


fn main() {
    println!("Welcome to the Guessing Game!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read user input from terminal");

    println!("You guessed: {}", guess);
}
