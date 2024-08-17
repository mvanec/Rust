use chrono::{NaiveDate, NaiveDateTime};
use mv_dbi::{model::project, DataObject, DbiDatabase};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Project {
    pub project_id: Uuid,
    pub project_name: String,
    pub project_date: NaiveDate,
    pub pay_rate: f64,
    pub project_duration: i64,
    pub total_pay: f64,
    pub tasks: Vec<ProjectTask>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct ProjectTask {
    pub task_id: Uuid,
    pub project_id: Uuid,
    pub task_name: String,
    pub task_duration: i64,
    pub task_date_time: NaiveDateTime,
    pub task_times: Vec<TaskTime>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct TaskTime {
    pub task_time_id: u64,
    pub task_id: Uuid,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

///
/// Prepare and save a Project to the database
///
pub async fn add_project(csv_project: &Project, dbi: &mut DbiDatabase) -> Result<u64, anyhow::Error> {
    let mut db_project: project::Project = project::Project::default();

    db_project.project_id = csv_project.project_id;
    db_project.project_date = csv_project.project_date;
    db_project.project_name = csv_project.project_name.clone();
    db_project.project_duration = csv_project.project_duration;
    db_project.pay_rate = csv_project.pay_rate;

    let mut dao = DataObject::Project(db_project.clone());
    let result = dbi.do_insert(&mut dao).await;
    match result {
        Ok(code) => {
            println!("Result {} for {}", code, &csv_project.project_name);
            return Ok(code)
        }
        Err(error) => return Err(anyhow::Error::new(error))
    }
}