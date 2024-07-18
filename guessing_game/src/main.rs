use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Welcome to the Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("{}",secret_number);

    let mut guess = String::new();

    loop{

        println!("Please enter your guess...");

        guess.clear();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input from terminal");

        let guess : u32 = match guess.trim().parse(){
            Ok(x) => x,
            Err(_) => {
                println!("Please enter a natural number!");
                continue;
            }
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too low"),
            Ordering::Equal => {
                println!("Congratulations, you win!");
                break;
            },
            Ordering::Greater => println!("Too big")
        }
    }
}
