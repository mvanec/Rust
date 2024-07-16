use std::thread;
use std::time::Duration;

use macros::*;

fn main() {
    println!("\n=========Running {}", function!());

    listing_16_1();
    listing_16_2();
    listing_16_2_5();
    listing_16_5();
    q1();
}

// Spawned thread may not get a chance to finish
fn listing_16_1() {
    println!("\n=========Running {}", function!());
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// Add a join call so the spawn can complete its mission
fn listing_16_2() {
    println!("\n=========Running {}", function!());
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

// Put the join call in the wrong spot
fn listing_16_2_5() {
    println!("\n=========Running {}", function!());
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// Use move to transfer ownership to the thread
fn listing_16_5() {
    println!("\n=========Running {}", function!());
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

#[allow(unused)]
fn q1() {
    println!("\n=========Running {}", function!());
    let mut n = 1;
    let t = thread::spawn(move || {
        n = n + 1;
        thread::spawn(move || {
            n = n + 1;
        })
    });
    n = n + 1;
    t.join().unwrap().join().unwrap();
    println!("{n}");
}