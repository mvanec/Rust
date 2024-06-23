#![allow(dead_code)]

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

fn deliver_order() {}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}

pub mod point {
    #[derive(Debug)]
    pub struct Point(pub i32, pub i32);
    impl Point {
        pub fn origin() -> Self {
            Point(0, 0)
        }
    }
}

pub mod a {
    pub mod b {
        pub fn f() {
            println!("b1");
        }
        pub mod c {
            pub fn f() {
                println!("c1");
            }
        }
    }
    pub fn entry() {
        super::b::c::f();
    }
}
pub mod b {
    pub fn f() {
        println!("b2");
    }
    pub mod c {
        pub fn f() {
            println!("c2");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        eat_at_restaurant()
    }

    #[test]
    fn q1() {
        crate::a::entry();
    }

    #[test]
    fn q2() {
        let mut p = point::Point::origin();
        p.0 += 1;
        println!("{p:?}");
    }
}
