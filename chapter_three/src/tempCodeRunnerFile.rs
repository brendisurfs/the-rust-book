use core::f32;


// SCALAR DATA TYPES

fn main() {

    // integer literals

    const dec: i32 = 98_222;

    const hex: i32 = 0xff;

    const octal: i32 = 0o77;
    // binary, byte etc.
    
}


// "integer overflow" occurs if you assign a too small bit size to a variable and pass a larger bit size into it. 

fn floating_point() { 

    // just use f64
    let x = 2.0;

    // f32 is basically the same, but not as accurate on modern processors. 
    let y: f32 = 3.0;
}


fn char() {
    // chars are defined by 'single quotes', rather than doubles. 
    let c= 'c';

    let heart_eyes_cat = '😻';
}

// rust chars are four bytes in size and represents a Unicode Scalar Value. This means that it can represent multiple chars that arent Latin based.


fn compounds() {

    // TUPLES 
    // a tuple is a way of grouping together a number of values with teh variety of type 
    let tup = (500, 6.4, 1);


    let z: (i32, f64, u8) = (500, 6.4, 1);

    // you can deconstruct via (.) notation
    let five_hundred = z.0;
}


// Array Type

fn array() {
    
    let a = [1,2,3,4,5];



    // why arrays?
    // arrays are useful when you want your data to be allocated 
    // on the stack rather than the heap.
    // OR when you want 
    // all elements in the array must have the same type.
    // arrays are different in Rust than other langs.
    // Rust Arrays are fixed length and cannot be modified. 

    // array type construction [i32; 5] can also be a construction format.
    let ff = [3; 5];
}


