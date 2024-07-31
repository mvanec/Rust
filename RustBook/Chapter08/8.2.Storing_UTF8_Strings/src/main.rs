fn main() {
    listing_8_15();
    listing_8_16();
    listing_8_18();
    listing_8_18_cont();
    string_len();
}

fn listing_8_15() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("String 8-15 = {s}");
}

fn listing_8_16() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
}

fn listing_8_18() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is {s3}");
}

fn listing_8_18_cont() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s = {}", &s);
}

fn string_len() {
    let spanish = "español";
    let esp = spanish.to_string();
    println!("Length of {} is {}", &esp, esp.len());
    let s = &esp[3..6];
    println!("s is {s}");

    for c in esp.chars() {
        println!("{c}");
    }
    for b in esp.bytes() {
        println!("{b}");
    }
}