// ownership
//===========================================================================

// if the data size is unknown at compile time, it must be stored on the heap.
// pushing to the stack is faster than allocating on the heap. 

// you can really think of it as a pool vs reserved spots in a restaurant? 
// accessing data in the heap is slower than on the stack because you need to follow the pointer. 


// ownership rules::
// each value in rust has a variable called its owner.
// there can only be one owner at a time.
// when the owner goes out of scope, the value will be dropped.

// String types:
//  - String::from("string");
// this requests space for the string that you are building right there.
// need to pair exactly one allocation and one free.
// resource aquisition: drop() . This aint C++ boi.

fn main() {
    let mut s = String::from("nice");

    s.push_str("nerds");
    println!("our string is {}", s);

    // integers stored on the stack because we know the size at compile time.
    // let x = 5;

    drop(s);

    // instead of copying the memory, s1 is no longer considered valid.
    let s1 = String::from("hello");
    let s2 = s1;
    
    // this is an example of a move. this is invalid here.
    // println!("{}", s1);

    println!("{}", s2);

    // clone is a visual indicator that there is a reason that the variable and data is being copied. 

    let s3 = s2.clone();

    println!("{}", s3);
}


