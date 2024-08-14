use std::{env, error::Error, fs::File, process};

use csv::ReaderBuilder;
use getopts::Options;

#[derive(Debug, Default)]
pub struct AppOptions {
    pub file: String,
    pub has_headers: bool,
}

fn main() {
    let opts = process_options();

    if let Err(err) = run(&opts) {
        println!("{}", err);
        process::exit(1);
    }
}

fn run(opts: &AppOptions) -> Result<(), Box<dyn Error>> {
    let file = File::open(&opts.file)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(opts.has_headers)
        .from_reader(file);

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

pub fn process_options() -> AppOptions {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt(
        "f",
        "file",
        "The name of the CSV file to be processed",
        "<file>",
    );
    opts.optflag("n", "no headers", "Indicate the file does not have headers");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(0);
    }

    let mut app_opts: AppOptions = AppOptions::default();
    app_opts.has_headers = true;
    let file = matches.opt_str("f");

    if let Some(fname) = file {
        app_opts.file = fname;
    } else if !matches.free.is_empty() {
        app_opts.file = matches.free[0].clone();
    } else {
        print_usage(&program, opts);
        std::process::exit(1);
    }

    if matches.opt_present("n") {
        app_opts.has_headers = false;
    }

    app_opts
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}
