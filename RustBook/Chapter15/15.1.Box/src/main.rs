#[allow(unused)]

use box_heap::List;
use box_heap::List::Cons;
use box_heap::List::Nil;

fn main() {
    listing_15_1();
    listing_15_3();
    q1();
}

fn listing_15_1() {
    let b = Box::new(5);
    println!("b = {}", b);
}

fn listing_15_3() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    let list: List<&str> = Cons("one", Box::new(Cons("two", Box::new(Cons("three", Box::new(Nil))))));
    println!("{:?}", list);
}

fn q1() {
    let mut n = 1;
    let b = Box::new(&mut n);
    **b += 1;
    println!("{}", n);
}