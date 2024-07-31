use macros::*;

#[derive(Debug)]
enum List {
    #[allow(unused)]
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    listing_15_18();
    listing_15_19();
}

fn listing_15_18() {
    println!("\n=========Running {}", function!());

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let _b = Cons(3, Rc::clone(&a));
    let _c = Cons(4, Rc::clone(&a));
    println!("a = {a:?}");
    println!("b = {_b:?}");
    println!("c = {_c:?}");
}

fn listing_15_19() {
    println!("\n=========Running {}", function!());

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}