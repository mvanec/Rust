mod models;
mod traits;
mod db;

use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

use traits::model_trait::load_from_csv;

fn handle_error(e: impl std::fmt::Display) {
    panic!("Error: {}", e.to_string());
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let projects_csv = env::var("PROJECTS_CSV").expect("PROJECTS_CSV must be set");
    let tasks_csv = env::var("TASKS_CSV").expect("TASKS_CSV must be set");
    let timings_csv = env::var("TIMINGS_CSV").expect("TIMINGS_CSV must be set");

    let pool = PgPool::connect(&database_url).await?;

    // Load projects from CSV
    load_from_csv(&projects_csv, &pool, |record| {
        let id = uuid::Uuid::parse_str(&record[0])
            .expect(&format!("Failed to parse project ID for record: {:?}", record));
        let name = record[1].clone();
        let start_date = chrono::NaiveDate::parse_from_str(&record[2], "%Y-%m-%d")
            .expect(&format!("Failed to parse project start date for record: {:?}", record));
        let end_date = chrono::NaiveDate::parse_from_str(&record[3], "%Y-%m-%d")
            .expect(&format!("Failed to parse project end date for record: {:?}", record));
        let cost = record[4].parse()
            .expect(&format!("Failed to parse project cost for record: {:?}", record));
        let stage = record[5].parse()
            .expect(&format!("Failed to parse project stage for record: {:?}", record));
        let phase = record[6].parse()
            .expect(&format!("Failed to parse project phase for record: {:?}", record));

        models::project::Project::new(id, name, start_date, end_date, cost, stage, phase)
    })
    .await
    .unwrap_or_else(handle_error);

    // Load tasks from CSV
    load_from_csv(&tasks_csv, &pool, |record| {
        let id = uuid::Uuid::parse_str(&record[0])
            .expect(&format!("Failed to parse task ID for record: {:?}", record));
        let project_id = uuid::Uuid::parse_str(&record[1])
            .expect(&format!("Failed to parse project ID for record: {:?}", record));
        let name = record[2].clone();
        let cost = record[3].parse()
            .expect(&format!("Failed to parse task cost for record: {:?}", record));

        models::task::Task::new(id, project_id, name, cost)
    })
    .await
    .unwrap_or_else(handle_error);

    // Load timings from CSV
    load_from_csv(&timings_csv, &pool, |record| {
        let project_id = uuid::Uuid::parse_str(&record[1])
            .expect(&format!("Failed to parse project ID for record: {:?}", record));
        let start_time = chrono::NaiveDateTime::parse_from_str(&record[2], "%Y-%m-%d %H:%M:%S%.f")
            .expect(&format!("Failed to parse start time for record: {:?}", record));
        let end_time = chrono::NaiveDateTime::parse_from_str(&record[3], "%Y-%m-%d %H:%M:%S%.f")
            .expect(&format!("Failed to parse end time for record: {:?}", record));

        models::timing::Timing::new(project_id, start_time, end_time)
    })
    .await
    .unwrap_or_else(handle_error);

    Ok(())
}
