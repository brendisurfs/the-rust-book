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
