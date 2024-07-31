use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// This as a deref trait to MyBox so the deref in
// listing_15_8() will work
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    listing_15_7();
    listing_15_8();
    listing_15_12();
    listing_15_13();
    q1();
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn listing_15_7() {
    let x = 7;
    let y = Box::new(x);

    assert_eq!(7, x);
    assert_eq!(7, *y);
    println!("y = {y}");
}

fn listing_15_8() {
    let x = 8;
    let y = MyBox::new(x);

    assert_eq!(8, x);
    assert_eq!(8, *y);
    println!("y = {y:?}");
}

// Without deref coercion
fn listing_15_12() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

// Without deref coercion
fn listing_15_13() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}

#[derive(Clone, Copy)]
struct AccessLogger(i32);
impl Deref for AccessLogger {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        println!("deref");
        &self.0
    }
}

// What does this print?
fn q1() {
  let n = AccessLogger(-1);
  let x = *n + 1;
  let _n2 = n;
  println!("{} {}", x, *n);
  println!("{}", *n);
}
