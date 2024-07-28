use macros::*;

fn main() {
    println!("\n=========Running {}", function!());
    listing_19_27();
    listing_19_27_a();
    listing_19_27_b();
    listing_19_27_c();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn listing_19_27() {
    println!("\n=========Running {}", function!());

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

#[allow(dead_code)]
fn listing_19_27_a() {
    println!("\n=========Running {}", function!());

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("------------------- Using a closure");
    for n in list_of_strings {
        println!("string: {n}")
    }

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("------------------- Using a function");
    for n in list_of_strings {
        println!("string: {n}")
    }
}

#[allow(dead_code)]
fn listing_19_27_b() {
    println!("\n=========Running {}", function!());
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..5).map(Status::Value).collect();
    println!("{:?}", list_of_statuses);
}

fn listing_19_27_c() {
    println!("\n=========Running {}", function!());
    let f = returns_closure();
    let y = f(5);
    println!("Result from closure: {y}");

}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}