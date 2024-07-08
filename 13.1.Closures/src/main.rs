use closures::listing_13_8;
use closures::run;
use closures::listing_13_7;

fn main() {
    run();
    listing_13_5();
    listing_13_6();
    listing_13_7();
    listing_13_8();
    q2();
}

fn listing_13_5() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

use std::thread;

fn listing_13_6() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut closure = move |x| { list.push(x); println!("From thread: {:?}", list) };
    thread::spawn(move || {closure(5);})
        .join()
        .unwrap();

    // println!("After spawn: {:?}", list);
}

fn q2() {
    let mut s = String::from("Hello");
    let add_suffix = |s: &mut String| s.push_str(" world");
    println!("{s}");
    add_suffix(&mut s);
}
