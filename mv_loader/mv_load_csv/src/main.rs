use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
use csv::ReaderBuilder;
use getopts::Options;
use mv_dbi::{utils::make_uuid, DbConfig, DbiDatabase};
use serde::{Deserialize, Deserializer};
use std::{env, error::Error, fs::File, process};
use time::macros::format_description;
use time::Time;

mod models;
use models::{combine_like_projects, Project, ProjectTask, TaskTime};

#[derive(Debug, Default)]
pub struct AppOptions {
    pub file: String,
    pub db_name: String,
    pub has_headers: bool,
}

// Date,Project,Pay Rate,Task ID,Start Time,End Time,Duration
#[allow(dead_code)]
#[derive(Debug, Default, Clone, Deserialize)]
struct Record {
    #[serde(rename = "Date", deserialize_with = "from_date_string")]
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

#[tokio::main]
async fn main() {
    let opts = process_options();

    if let Err(err) = run(&opts).await {
        println!("{}", err);
        process::exit(1);
    }
}

async fn run(opts: &AppOptions) -> Result<(), Box<dyn Error>> {
    let file = File::open(&opts.file)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(opts.has_headers)
        .from_reader(file);

    let mut records: Vec<Record> = Vec::new();
    for result in reader.deserialize() {
        let record: Record = result?;
        // println!("{:?}", &record);
        records.push(record);
    }
    let converted = convert_records(records)?;
    save_records_to_database(converted, opts).await?;
    Ok(())
}

/// Convert the CSV data into a hierarchy ready to put into the
/// database.
///
fn convert_records(records: Vec<Record>) -> Result<Vec<Project>, Box<dyn Error>> {
    let rec_iter = records.iter();

    let mut project: Project = Project::default();
    let mut task: ProjectTask = ProjectTask::default();

    let mut pflag = false;
    let mut tflag = false;
    let mut all_projects: Vec<Project> = Vec::with_capacity(rec_iter.len());

    for rec in rec_iter {
        match &rec.project {
            Some(project_name) => {
                if pflag {
                    let pd = TimeDelta::milliseconds(project.project_duration);
                    project.total_pay = (pd.num_minutes() as f64/60.0) * project.pay_rate;
                    project.tasks.push(task);
                    all_projects.push(project);
                    task = ProjectTask::default();
                    tflag = false;
                } else {
                    pflag = true;
                }
                project = Project::default();
                project.project_name = project_name.clone();
                project.project_date = rec.date.unwrap().into();
                project.pay_rate = rec.pay_rate.unwrap_or(0.0);
                let dt = project.project_date.format("%a %b %-d %C%y").to_string();
                let value = format!("{}{}", &project.project_name, &dt);
                project.project_id = make_uuid(&value);

                let dt_tm = NaiveDateTime::new(rec.date.unwrap().into(), rec.start_time);
                project.project_date = dt_tm;
                // println!("{:?}", &project.project_name);
            }
            None => (),
        }

        let start_time = NaiveDateTime::new(project.project_date.into(), rec.start_time.clone());
        let end_time = NaiveDateTime::new(project.project_date.into(), rec.end_time.clone());
        let duration = end_time - start_time;
        assert_eq!(duration.num_milliseconds(), rec.duration.num_milliseconds());
        match &rec.task_name {
            Some(task_name) => {
                if tflag {
                    project.tasks.push(task);
                } else {
                    tflag = true;
                }
                task = ProjectTask::default();
                task.project_id = project.project_id.clone();
                task.task_name = task_name.clone();
                task.task_date_time = start_time.clone();
                let value = format!("{}{}{}", &task.project_id.to_string(), &task.task_name, &start_time);
                task.task_id = make_uuid(&value);
                // println!("{:?}", &task.task_name);
            }
            None => (),
        }
        let mut task_time = TaskTime::default();
        task.task_duration += duration.num_milliseconds();
        project.project_duration += duration.num_milliseconds();
        task_time.start_time = start_time;
        task_time.end_time = end_time;
        task_time.task_id = task.task_id.clone();
        task.task_times.push(task_time);
    }
    let pd = TimeDelta::milliseconds(project.project_duration);
    project.total_pay = (pd.num_minutes() as f64/60.0) * project.pay_rate;
    project.tasks.push(task);
    all_projects.push(project);

    all_projects = combine_like_projects(all_projects);

    // println!("+++++++++++++++++++++++++++++++++++++++++++++++");
    // println!("{:#?}", &all_projects);

    Ok(all_projects)
}

///
/// Process the loaded and parsed CSV records into the database
///
async fn save_records_to_database(
    projects: Vec<Project>,
    opts: &AppOptions,
) -> Result<(), Box<dyn Error>> {
    let mut inserted: u64 = 0;

    let config = DbConfig::new(&opts.db_name);
    let mut db = DbiDatabase::new(config).await.unwrap();
    for project in projects {
        let x = models::add_project(&project, &mut db).await.unwrap();
        inserted += x;
    }
    println!("Read and inserted {inserted} projects");
    Ok(())
}

///
/// Parse the options from the command line arguments
///
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
    opts.optflag("n", "no-headers", "Indicate the file does not have headers");
    opts.optflag("h", "help", "print this help menu");
    opts.optopt(
        "d",
        "database",
        "The path and name of the sqlite3 database file",
        "<database>",
    );

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

    let db = matches.opt_str("d");
    if let Some(db_file) = db {
        app_opts.db_name = db_file;
    } else {
        app_opts.db_name = "sqlite::memory:".to_string();
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

///
/// Convert a string in "MM/DD/YYYY" format to a TimeDelta
///
fn from_date_string<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        return Ok(None);
    }

    let date_value = NaiveDate::parse_from_str(&s, "%-m/%-d/%Y")
        .expect(format!("Unable to parse date {s}").as_str());

    Ok(Some(date_value))
}

fn parse_time<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let format = format_description!("[hour padding:none repr:12]:[minute] [period]");
    let t = Time::parse(s, &format).unwrap();
    let nt =
        NaiveTime::from_hms_opt(t.hour().into(), t.minute().into(), t.second().into()).unwrap();
    Ok(nt)
}
