use std::io;
mod sort_data;

use sort_data::bucket_sort;

fn print_array(arr: &[i32]) {
    println!("{{");
    for (i, &num) in arr.iter().enumerate() {
        if i == arr.len() - 1 {
            print!("{}", num);
        } else {
            print!("{} ", num);
        }
    }
    println!("\n}}");
}

fn sort(arr: &mut Vec<i32>) {
    // Your code goes here!
    if arr.is_empty() {
        return;
    }
    let bucket_count = if arr.len() <= 5 { (arr.len() - 1) as i32 } else { 5 as i32 };
    bucket_sort(arr, bucket_count);
    //print_array(arr);  // Make sure to call this print after every iteration
}

fn main() {
    println!("Enter numbers separated by spaces: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // let input = String::from("0 5 9 1 7");

    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();

    print_array(&arr);
    let mut sortable_arr = arr.clone();
    sort(&mut sortable_arr);
    print_array(&sortable_arr);
}
