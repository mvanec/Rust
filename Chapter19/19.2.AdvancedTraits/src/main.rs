use std::fmt;

use advanced_traits::*;
use macros::*;

fn main() {
    println!("\n=========Running {}", function!());
    count_up();
    listing_19_14();
    listing_19_15();
    listing_19_18();
    listing_19_21();
    listing_19_22();
    listing_19_23();
    q3();
}

#[allow(unused_mut)]
fn count_up() {
    println!("\n=========Running {}", function!());
    let mut counter = Counter::new();
    count_iter(counter);
}

fn count_iter(mut iter: impl advanced_traits::Iterator) {
    loop {
        let value = iter.next();
        match value {
            Some(x) => println!("Iterated to {:?}", x),
            None => {
                println!("Finished iterating!");
                break;
            }
        }
    }
}

// Overload the "+" operatorr
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// The overload
impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
// Using the overloaded "+"
fn listing_19_14() {
    println!("\n=========Running {}", function!());
    let a = Point { x: 1, y: 0 };
    let b = Point { x: 2, y: 3 };
    let result = Point { x: 3, y: 3 };
    assert_eq!(a + b, result);

    println!("{a:?} + {b:?} = {result:?}");
}

// Using overloaded "+" with custom rhs values
fn listing_19_15() {
    println!("\n=========Running {}", function!());
    let m: Meters = Meters{ 0: 100 };
    let mm: Millimeters = Millimeters(1000);

    let result = mm + m;
    println!("{m}m + {mm}mm = {result}mm");
    println!("{mm}mm + {mm}mm = {}mm", mm + mm);
}

/*
 * Disambiguation
 */
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// Using disambiguation to specify which fly()
fn listing_19_18() {
    println!("\n=========Running {}", function!());
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

/*
 * Exposing hidden method with disambiguation
 */
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

/*
 * <Type as Trait>::function(receiver_if_method, next_arg, ...);
 */
fn listing_19_21() {
    println!("\n=========Running {}", function!());
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // let dog: Dog = Dog{};
    // println!("Dog's name is {}", dog.baby_name());
}

// Use Supertraits to require one trait's functionality
// in another
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

// Using the overloaded "+"
fn listing_19_22() {
    println!("\n=========Running {}", function!());
    let a = Point { x: 1, y: 0 };
    println!("a = {a}");
    a.outline_print();
}

/*
 * Implementing a trait on an external type by
 * wrapping it in a struct
 */
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn listing_19_23() {
    println!("\n=========Running {}", function!());
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

#[allow(dead_code)]
mod inner {
    pub trait A {
        fn f(&self) -> usize { 0 }
    }
    pub trait B {
        fn f(&self) -> usize { 1 }
    }
    pub struct P;
    impl A for P {}
    impl B for P {}
}

fn q3() {
    println!("\n=========Running {}", function!());
    use inner::{P, B};
    println!("{}", P.f());
}
