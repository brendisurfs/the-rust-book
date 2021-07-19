<<<<<<< HEAD
// chapter three!
// - this chapter is all about the common programming concepts of Rust (and what makes it special).

//  variables and mutability

fn main() {
    let mut counter = 0; 
    let result = loop {
        counter += 1; 
        
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is {}", result);
}
=======

use std::io;
fn main() { 
    let a = [1,2, 3,4,5];

    let mut index = String::new();

    io::stdin() 
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index
        // .trim returns a string slice with the whitespace removed.
        .trim()
        // .parse slices into another type.
        .parse()
        .expect("index entered was not a number");

    let el = a[index];

    println!("the value of the el at {} is: {}", index, el);
}
>>>>>>> 7bc8715a0ca9056488d77274c1c97fee593726f6
