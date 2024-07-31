use macros::*;

fn main() {
    println!("\n=========Running {}", function!());
    listing_18_11();
    listing_18_11_a();
    listing_18_11_b();
    listing_18_12();
    listing_18_14();
    listing_18_15();
    listing_18_16();
    listing_18_18();
    listing_18_19();
    listing_18_24();
    listing_18_26();
    listing_18_27();
    listing_18_29();
    q1();
    q2();
    q3();
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

#[allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Destructure an Enum
fn listing_18_15() {
    println!("\n=========Running {}", function!());
    // let msg = Message::ChangeColor(0, 160, 255);
    let msg = Message::Write("It was a dark and stormy night".to_string());

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
}

#[allow(unused)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[allow(unused)]
enum Message1816 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

// Destructure an Enum
fn listing_18_16() {
    println!("\n=========Running {}", function!());
    let msg = Message1816::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message1816::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message1816::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}

// Ignoring values
fn listing_18_18() {
    println!("\n=========Running {}", function!());
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

// Ignore some values
fn listing_18_19() {
    println!("\n=========Running {}", function!());
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}

// Ignore a range
fn listing_18_24() {
    println!("\n=========Running {}", function!());
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}

// Match Guards
fn listing_18_26() {
    println!("\n=========Running {}", function!());
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
}

// Match guard against external variable
fn listing_18_27() {
    println!("\n=========Running {}", function!());
    // let x = Some(5);
    let x = Some(10);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

// The @ operator
fn listing_18_29() {
    println!("\n=========Running {}", function!());
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

fn q1() {
    println!("\n=========Running {}", function!());
    let x = (0, 1);
    match x {
        (_, y) if y == 0 => println!("A"),
        (0, _) => println!("B"),
        _ => println!("C")
    }
}

fn q2() {
    println!("\n=========Running {}", function!());
    let a = [(0, 1)];
    let _ = a;
    let [..] = a;
    let [(_n, ..)] = a;
    //let (_, n) = a;
}

#[allow(unreachable_patterns)]
fn q3() {
    println!("\n=========Running {}", function!());
    let x = Some(&[0, 1]);
    match x {
        //Some(&[.., 1, ..]) => println!("A"),
        Some(&[0, 1]) | None => println!("B"),
        _ => println!("C")
    }
}