use macros::*;

use rc_leaks::{Node, NodeNoParent};

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    #[allow(unused)]
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    println!("\n=========Running {}", function!());
    listing_15_26();
    listing_15_27();
    listing_15_28();
    listing_15_29();
    q1();
}

fn listing_15_26() {
    println!("\n=========Running {}", function!());

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

fn listing_15_27() {
    println!("\n=========Running {}", function!());

    let leaf = Rc::new(NodeNoParent {
        value: 3,
        children: RefCell::new(vec![]),
    });
    println!("{:?}", leaf);

    let branch = Rc::new(NodeNoParent {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    println!("{:?}", branch);
}

fn listing_15_28() {
    println!("\n=========Running {}", function!());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // upgrade() here will return either Some(Rc<Node>(Node)) or None
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

fn listing_15_29() {
    println!("\n=========Running {}", function!());
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

fn q1() {
    println!("\n=========Running {}", function!());

    let r1 = Rc::new(0);
    let r4 = {
        let r2 = Rc::clone(&r1);
        Rc::downgrade(&r2)
    };
    let _r5 = Rc::clone(&r1);
    let _r6 = r4.upgrade();
    println!("{} {}", Rc::strong_count(&r1), Rc::weak_count(&r1));
}