// programming a guessing game

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    // create a random number.
    let secret_num: i32 = rand::thread_rng().gen_range(1..101);
    // println!("the random number is: {}", secret_num);

    println!("Guess the number!");
    
    // - loop time
    
    loop {
     
        // initialize a new String, make it mutable.
        let mut guess = String::new();

        println!("Please input your guess: ");

        // take the user input
        // - &mut guess just means that we can reference guess multiple places. 
        
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read input.");
        println!("Your guess: {}", guess);
        
        // - convert our String input into an i32.
        // -- this reusing of the variable name is called "shadowing"\
        // --- parse can parse various types, so we NEED to specify our type when calling the guess variable, rather than letting the program infer its type.

        let guess: i32 = guess.trim().parse().expect("please type a number!");

        // - match up with the secret num. 
        // "match" is an expression made of arms, a build of a pattern and code to run shoudl the value given at the beginning fit the arms pattern. 
        
        // - Ordering: an enum that has variants Less, Greater, Equal.  
        // - adding a loop!

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("nice you win!");
                break;   
            }
        }
    }
}
