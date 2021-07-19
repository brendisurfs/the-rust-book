fn main() {
    let s = String::from("nice");

    tk_own(s);

    let x = 5;

    make_c(x);
    // since i32 has trait Copy, its ok to use x after.
    // but String does not implement the Copy trait.
}

fn tk_own(str: String) {
    println!("{}", str);
}

fn make_c(sint: i32) {
    println!("{}", sint);
}