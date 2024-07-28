use macros::*;

fn main() {
    println!("\n=========Running {}", function!());
    q1();
    q2();
}

fn q1() {
    println!("\n=========Running {}", function!());
    println!("{:?}", expect_none(None));
}

fn q2() {
    println!("\n=========Running {}", function!());
    println!("{}", is_equal("Hello", "world"));
}

fn expect_none(x: Option<i32>)  /* ->  ! */ {
    match x {
        Some(n) => panic!("Expected none, found Some({n})"),
        None => (),
    }
}

fn is_equal<T: Eq + ?Sized>(t1: &T, t2: &T) -> bool {
    t1 == t2
}
