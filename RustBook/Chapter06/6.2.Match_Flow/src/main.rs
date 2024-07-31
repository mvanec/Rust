#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}


#[derive(Debug)]
enum Either {
  Left(usize),
  Right(String)
}
fn q3() {
  let x = Either::Right(String::from("Hello world"));
  let value = match &x {
    Either::Left(n) => n,
    Either::Right(s) => &s.len()
  };
  println!("{x:?} {value}");
}

fn list_68() {
    let opt: Option<String> = Some(String::from("Hello world"));

    match &opt {
        Some(_) => println!("Some!"),
        None => println!("None!"),
    };

    match &opt {
        Some(s) => println!("Some! {}", s),
        None => println!("None!"),
    };

    println!("{:?}", opt);
}

fn list_65() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(f) => Some(f + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five = {:?}", five);
    println!("six  = {:?}", six);
    println!("none = {:?}", none);
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
    list_65();
    list_68();
    q3();
}
