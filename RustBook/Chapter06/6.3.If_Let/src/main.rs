fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let config_none: Option<i32> = None;
    if let Some(max) = config_none {
        println!("The maximum is configured to be {}", max);
    }

    list_68();

    print_range_max(&Location::Range(0, 10));
    print_range_max(&Location::Point(32));
}

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

fn list_68() {
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    println!("Count is {count}");

    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("Count is {count}");
}

enum Location {
    Point(i32),
    Range(i32, i32)
  }
  fn print_range_max(loc: &Location) {
    // print the second field of Range, if loc is a Range
    if let Location::Range(_, n) = loc {
        println!("Range max is {n}");
    }
    else {
        println!("Range is not a range");
    }
  }