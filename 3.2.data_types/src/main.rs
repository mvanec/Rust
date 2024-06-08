fn main() {
    // addition
    let sum = 5 + 10;
    println!("Sum = {sum}");
    // subtraction
    let difference = 95.5 - 4.3;
    println!("Difference = {difference}");

    // multiplication
    let product = 4 * 30;
    println!("Product = {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("Qotient = {quotient}");
    println!("Truncated = {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("Remainder = {remainder}");

    // tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let f: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = f.0;
    let six_point_four = f.1;
    let one = f.2;
    println!(r"
               five hundred  : {five_hundred},
               six_point_four: {six_point_four},
               one:          : {one}");

    let t = ([1; 2], [3; 4]);
    let (a, b) = t;
    println!("{}, {} + {}", a[0] + t.1[0], a[0], t.1[0]);

}
