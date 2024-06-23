mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

pub mod parent {
    pub fn a() {}
    fn b() {}
    pub mod child {
        pub fn c() {}
    }
}
fn q2() {
    use parent::{child as alias, *};
    // ...
    a();
    parent::a();
    alias::c();
    child::c();
    parent::child::c();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        eat_at_restaurant();
    }
}
