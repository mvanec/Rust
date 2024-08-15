use chrono::{NaiveDate, NaiveTime, TimeDelta};
use csv::ReaderBuilder;
use getopts::Options;
use serde::{Deserialize, Deserializer};
use std::{env, error::Error, fs::File, process};

#[derive(Debug, Default)]
pub struct AppOptions {
    pub file: String,
    pub has_headers: bool,
}

// Date,Project,Pay Rate,Task ID,Start Time,End Time,Duration
#[allow(dead_code)]
#[derive(Debug, Default, Clone, Deserialize)]
struct Record {
    #[serde(rename = "Date", deserialize_with = "csv::invalid_option")]
    date: Option<NaiveDate>,
    #[serde(rename = "Project", deserialize_with = "csv::invalid_option")]
    project: Option<String>,
    #[serde(rename = "Pay Rate", deserialize_with = "csv::invalid_option")]
    pay_rate: Option<f64>,
    #[serde(rename = "Task ID", deserialize_with = "csv::invalid_option")]
    task_name: Option<String>,
    #[serde(rename = "Start Time", deserialize_with = "parse_time")]
    start_time: NaiveTime,
    #[serde(rename = "End Time", deserialize_with = "parse_time")]
    end_time: NaiveTime,
    #[serde(rename = "Duration", deserialize_with = "from_time_string")]
    duration: TimeDelta,
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

    for result in reader.deserialize() {
        let record: Record = result?;
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

///
/// Convert a string in "00:11:00" (HH:MM:SS) format to a TimeDelta
///
fn from_time_string<'de, D>(deserializer: D) -> Result<TimeDelta, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let dur: Vec<&str> = s.split(":").collect();

    let hh: i64 = dur[0].parse().unwrap_or_default();
    let mm: i64 = dur[1].parse().unwrap_or_default();
    let ss: i64 = dur[2].parse().unwrap_or_default();

    let mut minutes = (hh * 60) + mm;
    if ss > 0 {
        minutes += 1;
    }
    Ok(TimeDelta::minutes(minutes))
}

use time::macros::format_description;
use time::Time;

fn parse_time<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let format = format_description!("[hour padding:none repr:12]:[minute] [period]");
    let t = Time::parse(s, &format).unwrap();
    let nt = NaiveTime::from_hms_opt(t.hour().into(), t.minute().into(), t.second().into()).unwrap();
    Ok(nt)
}
