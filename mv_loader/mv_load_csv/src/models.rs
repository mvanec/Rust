use chrono::{NaiveDateTime, TimeDelta};
use mv_dbi::{model::{project, project_task, task_time}, DataObject, DbiDatabase};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Project {
    pub project_id: Uuid,
    pub project_name: String,
    pub project_date: NaiveDateTime,
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
pub async fn add_project(
    csv_project: &Project,
    dbi: &mut DbiDatabase,
) -> Result<u64, anyhow::Error> {
    let mut db_project: project::Project = project::Project::default();

    db_project.project_id = csv_project.project_id;
    db_project.project_date = csv_project.project_date.into();
    db_project.project_name = csv_project.project_name.clone();
    db_project.project_duration = csv_project.project_duration;
    db_project.pay_rate = csv_project.pay_rate;
    db_project.total_pay = csv_project.total_pay;

    let mut dao = DataObject::Project(db_project.clone());
    let result = dbi.do_insert(&mut dao).await;
    match result {
        Ok(code) => {
            add_project_task(&csv_project.tasks, dbi).await?;
            return Ok(code);
        }
        Err(error) => return Err(anyhow::Error::new(error)),
    }
}

async fn add_project_task(tasks: &Vec<ProjectTask>, dbi: &mut DbiDatabase) -> Result<u64, anyhow::Error> {

    for csv_task in tasks {
        let mut task = project_task::ProjectTask::default();
        task.task_id = csv_task.task_id;
        task.project_id = csv_task.project_id;
        task.task_name = csv_task.task_name.clone();
        task.task_date_time = csv_task.task_date_time;
        task.task_duration = csv_task.task_duration;

        let mut dao = DataObject::ProjectTask(task);
        let _result = dbi.do_insert(&mut dao).await?;
        let _result = add_task_times(&csv_task.task_times, dbi).await?;
    }
    Ok(0)
}

async fn add_task_times(times: &Vec<TaskTime>, dbi: &mut DbiDatabase) -> Result<u64, anyhow::Error> {

    for csv_time in times {
        let mut task_time = task_time::TaskTime::default();
        task_time.task_time_id = 0;
        task_time.task_id = csv_time.task_id;
        task_time.start_time = csv_time.start_time;
        task_time.end_time = csv_time.end_time;
        let mut dao = DataObject::TaskTime(task_time);
        let _result = dbi.do_insert(&mut dao).await?;
    }
    Ok(0)
}

pub fn combine_like_projects(all_projects: Vec<Project>) -> Vec<Project>{
    let mut projects_map: HashMap<Uuid, Project> = HashMap::with_capacity(all_projects.capacity());

    for project in all_projects {
        let key = project.project_id;
        if projects_map.contains_key(&key) {
            let saved = projects_map.get_mut(&key).unwrap();
            for task in &project.tasks {
                saved.project_duration += task.task_duration;
                saved.tasks.push(task.clone());
            }
            let pd = TimeDelta::milliseconds(saved.project_duration);
            saved.total_pay = (pd.num_minutes() as f64/60.0) * saved.pay_rate;
            continue;
        }
        projects_map.insert(key, project);
    }
    let mut projects_combined: Vec<Project> = projects_map.into_iter().map(|(_id, project)| project).collect();
    projects_combined.sort_by(|a, b| a.project_date.cmp(&b.project_date));
    projects_combined
}