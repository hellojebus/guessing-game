use std::io;
use rand::Rng;

fn main() {
    println!("Guessing game");
    println!("Enter a number:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
