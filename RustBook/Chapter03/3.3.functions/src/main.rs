fn main() {
    println!("Hello, world!");

    // Functions with arguments
    another_function(33);
    print_labeled_measurement(5, 'h');

    // Expressions
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    // Return values
    let x = five();
    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}
