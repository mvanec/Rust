use macros::*;

#[derive(Debug)]
enum List {
    #[allow(unused)]
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("\n=========Running {}", function!());
    listing_15_24();
    q2();
}

fn listing_15_24() {
    println!("\n=========Running {}", function!());

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

use std::cell::UnsafeCell;

#[derive(Debug)]
struct BadRefCell<T>(UnsafeCell<T>);
impl<T> BadRefCell<T> {
    pub fn borrow_mut(&self) -> &mut T {
        unsafe { &mut *self.0.get() }
    }
}

fn q2() {
    println!("\n=========Running {}", function!());

    let v = BadRefCell(UnsafeCell::new(vec![1, 2, 3]));

    let v1 = v.borrow_mut();
    let n = &v1[0];
    v.borrow_mut().push(0);
    println!("{n}");

    // let v1 = v.borrow_mut();
    // let v2 = v.borrow_mut();
    // v1.push(0);
    // v2.push(0);
    // println!("{:?}", v.borrow_mut());

    // drop(v.borrow_mut());
    // drop(v.borrow_mut());

    // v.borrow_mut().push(0);
    // let n = v.borrow_mut()[0];
    // println!("{n}");
}
