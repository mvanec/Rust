use chrono::{NaiveDate, NaiveDateTime};
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
