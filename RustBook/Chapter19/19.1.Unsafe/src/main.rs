use macros::*;

fn main() {
    listing_19_3();
    listing_19_4(true);
    listing_19_4(false);
    listing_19_8();
    call_from_c();
    listing_19_10();
    q2();
}

fn listing_19_3() {
    println!("\n=========Running {}", function!());
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

fn listing_19_4(custom: bool) {
    println!("\n=========Running {} => {custom}", function!());
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = match custom {
        true  => r.split_at_mut(3),
        false => split_at_mut(r, 3),
    };

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    println!("a = {:?}", a);
    println!("b = {:?}", b);
}

use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn listing_19_8() {
    println!("\n=========Running {}", function!());
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("\n=========Running {}", function!());
    println!("Just called a Rust function from C!");
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn listing_19_10() {
    println!("\n=========Running {}", function!());
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn q2() {
    println!("\n=========Running {}", function!());
    let mut v = Vec::with_capacity(4);
    for i in 0 .. 3 {
        v.push(i);
    }
    let n = &v[0] as *const i32;
    v.push(4);
    println!("{}", unsafe { *n });
}