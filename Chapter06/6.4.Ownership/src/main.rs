fn main() {
    println!("Hello, world!");
}

/// Makes a string to separate lines of text,
/// returning a default if the provided string is blank
fn make_separator(user_str: &str) -> String {
    if user_str == "" {
        let default = "=".repeat(10);
        default
    } else {
        user_str.to_string()
    }
}

/// Gets the string out of an option if it exists,
/// returning a default otherwise
fn get_or_default(arg: &Option<String>) -> String {
    match arg {
        None => String::new(),
        Some(s) => s.clone()
    }
}
