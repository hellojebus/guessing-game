use std::io;
use rand::Rng;

fn main() {
    println!("Guessing game");
    
    let secret_num = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_num);

    println!("Enter a number:");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
