fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn main() {
    let x = true;
    read(x);

    let a = Box::new([0; 1_000_000]);
    let mut b = a;
    b[10] = 5;
    println!("a[10] = {}, b[10] = {}", b[10], b[10]);

    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");

    let s = String::from("hello");
    let s2;
    let b = false;
    if b {
      s2 = s;
    }
    // println!("{}", s);

}

