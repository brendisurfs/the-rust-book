
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