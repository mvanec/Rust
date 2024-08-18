#![allow(unused)]
// models.rs
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::error::BoxDynError;
use sqlx::query::Query;
use sqlx::sqlite::Sqlite;
use sqlx::sqlite::SqliteArguments;
use sqlx::sqlite::SqliteRow;
use sqlx::sqlite::SqliteValueRef;
use sqlx::Column;
use sqlx::Decode;
use sqlx::Error;
use sqlx::FromRow;
use sqlx::Row;
use uuid::Uuid;

use crate::database::query::DbObject;
use crate::DataObject;

#[derive(Debug, Clone, Default, FromRow, PartialEq, Deserialize, Serialize)]
#[sqlx(rename_all = "PascalCase")]
pub struct ProjectTask {
    pub task_id: Uuid,
    pub project_id: Uuid,
    pub task_name: String,
    pub task_duration: i64,
    pub task_date_time: NaiveDateTime,
}

impl DbObject<Sqlite, ProjectTask> for ProjectTask {
    async fn insert_one(pool: &sqlx::Pool<Sqlite>, dbo: &ProjectTask) -> Result<u64, Error> {
        let mut tx = pool.begin().await?;
        let sql = "INSERT INTO ProjectTasks (
            TaskId,
            ProjectId,
            TaskName,
            TaskDuration,
            TaskDateTime
        ) VALUES (
            $1, $2, $3, $4, $5
        )";

        let query = sqlx::query(sql)
            .bind::<Uuid>(dbo.task_id)
            .bind::<Uuid>(dbo.project_id)
            .bind::<String>(dbo.task_name.clone())
            .bind::<i64>(dbo.task_duration)
            .bind::<NaiveDateTime>(dbo.task_date_time)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(query.rows_affected())
    }

    async fn retrieve_all(pool: &sqlx::Pool<Sqlite>) -> Result<Vec<ProjectTask>, Error> {
        let sql = "SELECT
            TaskId,
            ProjectId,
            TaskName,
            TaskDuration,
            TaskDateTime
        FROM ProjectTasks
        ORDER BY ProjectId, TaskDateTime ASC";
        let records: Vec<ProjectTask> = sqlx::query_as(&sql).fetch_all(pool).await?;
        Ok(records)
    }

    async fn retrieve_some(
        pool: &sqlx::Pool<Sqlite>,
        uuid: &Uuid,
    ) -> Result<Vec<ProjectTask>, Error> {
        let sql = "SELECT
            TaskId,
            ProjectId,
            TaskName,
            TaskDuration,
            TaskDateTime
        FROM ProjectTasks
        WHERE ProjectId = ?
        ORDER BY ProjectId, TaskDateTime ASC";
        let records: Vec<ProjectTask> = sqlx::query_as(&sql).bind(&uuid).fetch_all(pool).await?;
        Ok(records)
    }

    async fn retrieve_one(&mut self, pool: &sqlx::Pool<Sqlite>) -> Result<(), Error> {
        let sql = "SELECT
            TaskId,
            ProjectId,
            TaskName,
            TaskDuration,
            TaskDateTime
        FROM ProjectTasks
        WHERE TaskId = ?";

        let row: SqliteRow = sqlx::query(&sql).bind(self.task_id).fetch_one(pool).await?;
        let temp_project = ProjectTask::from_row(&row)?;

        self.task_id = temp_project.task_id;
        self.project_id = temp_project.project_id;
        self.task_name = temp_project.task_name;
        self.task_date_time = temp_project.task_date_time;
        self.task_duration = temp_project.task_duration;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::*;
    use crate::DataCollection;
    use crate::DbConfig;
    use crate::DbObject;
    use crate::DbiDatabase;
    use chrono::NaiveDate;
    use chrono::NaiveDateTime;
    use sqlx::migrate::Migrator;
    use sqlx::Error;
    use uuid::Uuid;

    use super::ProjectTask;
    use crate::Project;

    fn make_project() -> Project {
        let mut project = Project::default();
        // Date string  "Sat Aug 10 2024"
        // Uuid = 990950c8-e2e2-55f0-8217-ac3b08d2fbae
        project.project_name = "A project Name again".to_string();
        project.project_date =
            NaiveDate::from_ymd_opt(2024, 8, 10).expect("Tried to create an invalid date");
        project.project_duration = 0;
        project.pay_rate = 40.0;
        project.total_pay = 0.0;

        let dt = project.project_date.format("%a %b %-d %C%y").to_string();
        let value = format!("{}{}", &project.project_name, &dt);
        project.project_id = make_uuid(&value);
        project
    }

    fn make_task(project_id: Uuid, time_diff: i64) -> ProjectTask {
        let mut task = ProjectTask::default();
        // Date string  "Sat Aug 10 2024"
        let dt_str = format!("{}-{}-{} {}:{}:{}", 2024, "08", 10, 12 + time_diff, 30, 30);
        let name = format!("Task {:2}", time_diff + 1);
        task.project_id = project_id;
        task.task_name = name;
        task.task_duration = 0;
        task.task_date_time = NaiveDateTime::parse_from_str(&dt_str, "%Y-%m-%d %H:%M:%S").unwrap();

        let value = format!("{}{}", &task.project_id.to_string(), &task.task_name);
        task.task_id = make_uuid(&value);
        task
    }

    fn modify_project(project: &mut Project, num: i32) {
        project.project_name = project.project_name.clone() + "0" + &num.to_string().as_str();
        let dt = &project.project_date.format("%a %b %-d %C%y").to_string();
        let value = format!("{}{}", &project.project_name, &dt);
        project.project_id = make_uuid(&value);
    }

    async fn setup() -> Result<DbiDatabase, Error> {
        let config = DbConfig::new("sqlite::memory:");
        let mut db = DbiDatabase::new(config).await?;
        Ok(db)
    }

    #[tokio::test]
    async fn test_insert_one_task() -> Result<(), Error> {
        let mut db = setup().await?;
        let project = make_project();
        let num_rows = Project::insert_one(&db.pool, &project).await?;
        assert_eq!(num_rows, 1);

        let expected = make_task(project.project_id.clone(), 0);

        let num_rows = ProjectTask::insert_one(&db.pool, &expected).await?;
        assert_eq!(num_rows, 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_select_one_task() -> Result<(), Error> {
        let mut db = setup().await?;
        let project = make_project();
        let num_rows = Project::insert_one(&db.pool, &project).await?;
        assert_eq!(num_rows, 1);

        let expected = make_task(project.project_id.clone(), 0);
        let mut actual = ProjectTask::default();
        actual.task_id = expected.task_id.clone();

        let num_rows = ProjectTask::insert_one(&db.pool, &expected).await?;
        assert_eq!(num_rows, 1);

        actual.retrieve_one(&db.pool).await?;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_select_all_task() -> Result<(), Error> {
        let mut db = setup().await?;
        let project = make_project();
        let num_rows = Project::insert_one(&db.pool, &project).await?;
        assert_eq!(num_rows, 1);

        let mut expected: Vec<ProjectTask> = Vec::with_capacity(3);

        for i in 0..3 {
            let a_task = make_task(project.project_id.clone(), i);
            let res = ProjectTask::insert_one(&db.pool, &a_task).await?;
            assert_eq!(res, 1);
            expected.push(a_task);
        }

        let actual = ProjectTask::retrieve_all(&db.pool).await?;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_select_some_task() -> Result<(), Error> {
        let mut db = setup().await?;
        let mut p1 = make_project();
        let mut p2 = make_project();
        modify_project(&mut p1, 1);
        modify_project(&mut p2, 2);

        let num_rows = Project::insert_one(&db.pool, &p1).await?;
        assert_eq!(num_rows, 1);
        let num_rows = Project::insert_one(&db.pool, &p2).await?;
        assert_eq!(num_rows, 1);

        let mut expected: Vec<ProjectTask> = Vec::with_capacity(3);

        for i in 0..3 {
            let a_task = make_task(p1.project_id.clone(), i);
            let res = ProjectTask::insert_one(&db.pool, &a_task).await?;
            assert_eq!(res, 1);
            expected.push(a_task);
        }

        for i in 3..6 {
            let a_task = make_task(p2.project_id.clone(), i);
            let res = ProjectTask::insert_one(&db.pool, &a_task).await?;
            assert_eq!(res, 1);
        }

        let actual = ProjectTask::retrieve_some(&db.pool, &p1.project_id).await?;
        assert_eq!(actual, expected);

        Ok(())
    }
}
