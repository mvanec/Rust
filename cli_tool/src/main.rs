#[allow(unused_imports)]
use anyhow::{Context, Result};

use colored::*;
use indicatif::ProgressBar;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::{fs::File, io::BufRead, io::BufReader};

use clap::{Arg, ArgAction, Command};

fn main() -> Result<()> {
    env_logger::init();

    let matches = Command::new("cli_tool")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Does awesome things")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("count")
                .about("Counts characters, words, or lines")
                .arg(
                    Arg::new("chars")
                        .short('c')
                        .long("chars")
                        .help("Count characters")
                        .action(ArgAction::SetTrue), // Added action
                )
                .arg(
                    Arg::new("words")
                        .short('w')
                        .long("words")
                        .help("Count words")
                        .action(ArgAction::SetTrue), // Added action
                )
                .arg(
                    Arg::new("lines")
                        .short('l')
                        .long("lines")
                        .help("Count lines")
                        .action(ArgAction::SetTrue), // Added action
                )
                .arg(
                    Arg::new("input")
                        .help("Input file") // Changed from .about() to .help()
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("count", sub_m)) => {
            let input = sub_m
                .get_one::<String>("input")
                .expect("Input file is required"); // Replaced .value_of()
            let count_chars = sub_m.get_flag("chars"); // Replaced .is_present()
            let count_words = sub_m.get_flag("words"); // Replaced .is_present()
            let count_lines = sub_m.get_flag("lines"); // Replaced .is_present()

            // Call function to perform counting
            perform_counting(input, count_chars, count_words, count_lines)?;
        }
        _ => unreachable!("No valid subcommand was used"),
    }

    Ok(())
}

fn perform_counting(input: &str, chars: bool, words: bool, lines: bool) -> Result<()> {
    info!("Starting counting for file: {}", input);
    let file = File::open(input).with_context(|| format!("Could not open file '{}'", input))?;
    let metadata = file.metadata()?;

    #[allow(unused)]
    let total_size = metadata.len();
    let reader = BufReader::new(file);

    let pb = ProgressBar::new(2 /*total_size */);

    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;

    let millis = std::time::Duration::from_millis(1000);

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        if lines {
            line_count += 1;
        }
        if words {
            word_count += line.split_whitespace().count();
        }
        if chars {
            char_count += line.len();
        }
        std::thread::sleep(millis);
        pb.inc(1);
    }
    pb.finish_with_message("Done");

    if lines {
        println!("Lines: {}", line_count.to_string().green());
    }
    if words {
        println!("Words: {}", word_count.to_string().red());
    }
    if chars {
        println!("Characters: {}", char_count.to_string().blue());
    }

    Ok(())
}
