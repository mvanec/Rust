fn main() {
    let time_string: &str = "2023-01-05 09:00:00.123";
    let ts = chrono::NaiveDateTime::parse_from_str(time_string, "%Y-%m-%d %H:%M:%S%.f").unwrap();
    println!("String of time is {}", &ts);
}
