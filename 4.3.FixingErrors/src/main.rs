// have the caller provide a "slot" to put the string using a mutable reference
fn return_a_string(output: &mut String) {
    output.replace_range(.., "Hello world");
}

// Move ownership out of the method
fn return_b_string() -> String {
    let s = String::from("b Hello world");
    s
}

// Live-forever string literal
fn return_c_string() -> &'static str {
    "Hello world"
}

// defer borrow-checking to runtime by using garbage collection
// using reference counting
use std::rc::Rc;
fn return_d_string() -> Rc<String> {
    let s = Rc::new(String::from("RC Hello world"));
    Rc::clone(&s)
}

fn main() {
    let mut s: String = "Goodby".to_string();
    return_a_string(&mut s);
    println!("s is now {s}");

    s = return_b_string();
    println!("s is now {s}");

    let b: &str = return_c_string();
    println!("b is now {b}");

    s = return_d_string().to_string();
    println!("s is now {s}");
}
