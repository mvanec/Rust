use macros::*;
use adv_macros::*;
use advanced_macros::HelloMacro;
// use advanced_macros_derived::HelloMacro;
use advanced_macros_derived::*;

fn main() {
    println!("\n=========Running {}", function!());
    listing_19_28();
    listing_19_30();
    index();
}

fn listing_19_28() {
    println!("\n=========Running {}", function!());
    let v: Vec<&str> = adv_vec!["a", "b", "c"];
    println!("{:?}", v);
}

#[derive(HelloMacro)]
struct Pancakes;

fn listing_19_30() {
    println!("\n=========Running {}", function!());
    Pancakes::hello_macro();
}

//#[route(GET "/")]
fn index() {
    println!("\n=========Running {}", function!());
    println!("Index");
}

// macro_rules! manylet {
//     ( $( $i:ident ),* = $e:expr ) => {
//         $(
//             let mut $i = $e;
//         )*
//     }
// }
// fn q1() {
//     println!("\n=========Running {}", function!());
//     let mut s = String::from("A");
//     manylet!(x, y = s);
//     x.push_str("B");
//     println!("{x}{y}");
// }
