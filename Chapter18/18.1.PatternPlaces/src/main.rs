use macros::*;

fn main() {
    println!("\n=========Running {}", function!());
    listing_18_1();
    listing_18_2();
    listing_18_3();
    listing_18_7();
}

fn listing_18_1() {
    println!("\n=========Running {}", function!());
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn listing_18_2() {
    println!("\n=========Running {}", function!());
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn listing_18_3() {
    println!("\n=========Running {}", function!());
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn listing_18_7() {
    println!("\n=========Running {}", function!());
    let point = (3, 5);
    print_coordinates(&point);
    let x: &[(i32, i32)] = &[(0, 1)];
    println!("x = {:?}", x[0]);
}