use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to my guessing game");

    loop{
        println!("What is your guess?");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Couldn't get user input");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}