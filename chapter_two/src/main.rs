// programming a guessing game

use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess: ");

    // initialize a new String, make it mutable.
    let mut guess = String::new();

    // take the user input
    // - &mut guess just means that we can reference guess multiple places. 
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read input.");

    println!("Your guess: {}", guess);    
}
