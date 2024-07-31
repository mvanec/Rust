#![allow(unused_variables)]

use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    listing_9_3();
    listing_9_5_a();

    match read_username_from_file() {
        Ok(username) => println!("Read user '{username}'"),
        Err(e) => eprintln!("Error: {:?}", e),
    };

    match read_username_from_file_9_7() {
        Ok(username) => println!("Read user '{username}'"),
        Err(e) => eprintln!("Error: {:?}", e),
    };

    match read_username_from_file_9_9() {
        Ok(username) => println!("Read user '{username}'"),
        Err(e) => eprintln!("Error: {:?}", e),
    };
}

fn listing_9_3() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn listing_9_5_a() {
    let greeting_file = File::open("hello95a.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello95a.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello96.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_9_7() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_9_9() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}