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

fn borrowed() {
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

// Unsafe Fixing
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

/// Rounds all the floats in a vector to the nearest integer, in-place
fn round_in_place(v: &mut Vec<f32>) {
    for n in v {
        *n = n.round();
    }
}

fn main() {
    borrowed();

    let name = vec![String::from("Ferris")];
    let first = &name[0];
    let rslt = stringify_name_with_title(&name);
    println!("First; {}, Whole = {rslt}", first);

    let mut v: Vec<f32> = vec![1.2, 2.6, 6.5, 8.0];
    round_in_place(&mut v);
    println!("{:?}", v);
    m();
}

fn m() {
    let mut point = [0, 1];
    let mut x = point[0];
    let y = &mut point[1];
    x += 1;
    *y += 1;
    println!("{} {}", point[0], point[1]);
}
