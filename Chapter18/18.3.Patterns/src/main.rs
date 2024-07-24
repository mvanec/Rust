use macros::*;

fn main() {
    println!("\n=========Running {}", function!());
    listing_18_11();
    listing_18_11_a();
    listing_18_11_b();
    listing_18_12();
    listing_18_14();
}

fn listing_18_11() {
    println!("\n=========Running {}", function!());

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // Introduces a shadow variable
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

// Multi-pattern match
fn listing_18_11_a() {
    println!("\n=========Running {}", function!());
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// Range pattern match
fn listing_18_11_b() {
    println!("\n=========Running {}", function!());
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

// De-structuring structs
struct Point {
    x: i32,
    y: i32,
}

fn listing_18_12() {
    println!("\n=========Running {}", function!());
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let q = Point { x: 0, y: 7 };

    let Point { x, y } = q;
    assert_eq!(0, x);
    assert_eq!(7, y);

}

fn listing_18_14() {
    println!("\n=========Running {}", function!());
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    let p = Point { x: 0, y: 0 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}