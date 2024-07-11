//!
//! # Mini Grep
//!
//! `minigrep` is a streamlined implementation of finding
//! needles in haystacks.
//!
use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

///
/// Holds the configuration information for the application
///
/// * **program**: The name of the program being executed
/// * **query**:   The string that the program should look for
/// * **file_path**: The file that the program will look for `query` in
/// * **ignore_case**: Whether or not the search is case-sensitive
///
pub struct Config {
    pub program: String,
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {

    ///
    /// Build the Config object using the provided `args` iterator. Typically
    /// this will come from command-line arguments.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {

        let program = match args.next() {
            Some(arg) => arg,
            None => return Err("Couldn't get program name"),
        };

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            program,
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
