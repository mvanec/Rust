use macros::*;

fn main() {
    listing_19_3();
    listing_19_4(true);
    listing_19_4(false);
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
