use std::collections::HashMap;

fn main() {
    listing_8_20();
    listing_8_21();
    listing_8_22();
    listing_8_23();
    listing_8_24();
    listing_8_25();
    quiz_2();
    quiz_6()
}

// Create a HashMap and store values
fn listing_8_20() {
    println!("\n=========== listing_8_20");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Scores:\n{:?}", &scores);
}

// Create a HashMap and access values
fn listing_8_21() {
    println!("\n=========== listing_8_21");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Scores for {team_name} is {score}");

    for (key, value) in &scores {
        println!("{key:<10}: {value:>5}");
    }
}

// Values a moved and owned by HashMap
fn listing_8_22() {
    println!("\n=========== listing_8_22");
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("Color is {}", &field_value);
}

// Overwriting values
fn listing_8_23() {
    println!("\n=========== listing_8_23");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    println!("{:?}", scores);

    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}

// Check if key exists before overwriting
fn listing_8_24() {
    println!("\n=========== listing_8_24");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

// Update value based on old value
fn listing_8_25() {
    println!("\n=========== listing_8_25");
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn quiz_2() {
    println!("\n=========== quiz_2");
    let mut h: HashMap<char, Vec<usize>> = HashMap::new();
    for (index, c) in "hello!".chars().enumerate() {
        h.entry(c).or_insert(Vec::new()).push(index);
    }
    let mut sum = 0;
    for index in h.get(&'l').unwrap() {
        sum += *index;
    }
    println!("{} for {:?}", sum, h);
}

fn quiz_6() {
    println!("\n=========== quiz_6");
    let mut list: Vec<String> = vec![String::from("a"), String::from("b")];
    reverse(&mut list);
    println!("{:?}", list);
}

// fn reverse(v: &mut Vec<String>) {
//     let n = v.len();
//     for i in 0 .. n / 2 {
//         let s1 = v[i].clone();
//         let s2 = v[n - i - 1].clone();
//         v[i] = s2;
//         v[n - i - 1] = s1;
//     }
// }

// fn reverse(v: &mut Vec<String>) {
//     let n = v.len();
//     let mut v2 = v.clone();
//     for i in 0 .. n / 2 {
//         std::mem::swap(&mut v[i], &mut v2[n - i - 1]);
//     }
// }

fn reverse(v: &mut Vec<String>) {
    let n = v.len();
    for i in 0 .. n / 2 {
        let p1 = &mut v[i] as *mut String;
        let p2 = &mut v[n - i - 1] as *mut String;
        unsafe { std::ptr::swap_nonoverlapping(p1, p2, 1); }
    }
}
