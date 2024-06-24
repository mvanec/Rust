fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    list_8_7();
    list_8_8();
    find_until(&vec![1, 2, 3], 1, 4);
    iter();
    list_8_9();
    quiz_2();
}

fn list_8_7() {
    let v = vec![100, 32, 57];
    for n_ref in &v {
        // n_ref has type &i32
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }
}

fn list_8_8() {
    let mut v = vec![100, 32, 57];
    println!("v = {:?}", &v);
    for n_ref in &mut v {
        // n_ref has type &mut i32
        *n_ref += 50;
    }
    println!("v = {:?}", &v);
}

fn find_until(v: &Vec<i32>, n: i32, til: usize) -> Option<usize> {
    for i in 0..til {
        if v[i] == n {
            return Some(i);
        }
    }
    return None;
}

fn iter() {
    use std::ops::Range;
    #[allow(unused_mut)]
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: Range<usize> = 0..v.len();
    let i1: usize = iter.next().unwrap();
    let n1: &i32 = &v[i1];
    println!("il: {i1}, nl: {n1}");
}

fn list_8_9() {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Row = {:?}", &row);
}

fn quiz_2() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let mut v2: Vec<&mut i32> = Vec::new();
    for i in &mut v {
        v2.push(i);
    }
    *v2[0] = 5;
    let a = *v2[0];
    let b = v[0];
    println!("{a} {b}");
}
