#![allow(dead_code)]
#![allow(unused_variables)]

fn foo(x: &i32) {
    println!("{x}");
}
fn main() {
    let x = null;
    foo(x);
}
