fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} {}", m1, m2);
    println!("{s}!");

    deref();
    deref_again();
    deref_count();
    vec_ref();
    vec_mut_ref();
    let mut v: Vec<char> = vec!['b', 'a', 'd'];
    ascii_capitalize(&mut v);
    println!("Capitalized result: {:?}", v);
    mut_s();
    vec_life();
    final_q();
}

// The reference is borrowed, not moved
fn greet(g1: &String, g2: &String) {
    // note the ampersands
    println!("{} {}!", g1, g2);
}

fn deref() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x; // *x reads the heap value, so a = 1
    *x += 1; // *x on the left-side modifies the heap value,
             //     so x points to the value 2

    let r1: &Box<i32> = &x; // r1 points to x on the stack
    let b: i32 = **r1; // two dereferences get us to the heap value

    let r2: &i32 = &*x; // r2 points to the heap value directly
    let c: i32 = *r2; // so only one dereference is needed to read it
    println!("a = {a}, b =  {b}, c = {c}");
}

fn deref_again() {
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs(); // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs(); // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len(); // implicit reference
    assert_eq!(s_len1, s_len2);
}

fn deref_count() {
    let x = Box::new(1);
    let y = Box::new(&x);
    // Deref to type Box<&Box<i32>>
    println!("y = {}", ***y);
}

fn vec_ref() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    println!("Third element is {}", *num);
    v.push(4);
    println!("Third element is still {}", v[2]);
}

fn vec_mut_ref() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);
}

fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];
    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        v[0] = up;
    } else {
        println!("Already capitalized: {:?}", v);
    }
}

fn mut_s() {
    let mut s = String::from("Hello");
    let t = &mut s;
    /* here */
    t.push_str(" world");
    println!("{}", s);
}

fn get_first(v: &Vec<String>) -> &str {
    &v[0]
}

fn vec_life() {
    let mut strs = vec![String::from("A"), String::from("B")];
    let first = get_first(&strs);

    println!("strings = {:?}", strs);

    if first.len() > 0 {
        strs.push(String::from("C"));
    }
}

fn incr(n: &mut i32) {
    *n += 1;
}

fn final_q() {
    let mut n: i32 = 1;
    incr(&mut n);
    println!("{n}");
}
