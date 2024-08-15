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
pub struct TaskTime {
    pub task_time_id: u64,
    pub task_id: Uuid,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

impl DbObject<Sqlite, TaskTime> for TaskTime {
    async fn insert_one(pool: &sqlx::Pool<Sqlite>, dbo: &TaskTime) -> Result<u64, Error> {
        let mut tx = pool.begin().await?;
        let sql = "INSERT INTO TaskTimes (
            TaskId,
            StartTime,
            EndTime
        ) VALUES (
            $1, $2, $3
        )";

        let query = sqlx::query(sql)
            .bind::<Uuid>(dbo.task_id)
            .bind::<NaiveDateTime>(dbo.start_time)
            .bind::<NaiveDateTime>(dbo.end_time)
            .execute(&mut *tx)
            .await?;

        let row: (i64,) = sqlx::query_as("SELECT last_insert_rowid()")
            .fetch_one(&mut *tx)
            .await?;
        let row_id = row.0 as u64;

        tx.commit().await?;

        Ok(row_id)
    }

    async fn retrieve_all(pool: &sqlx::Pool<Sqlite>) -> Result<Vec<TaskTime>, Error> {
        let sql = "SELECT
            TaskTimeId,
            TaskId,
            StartTime,
            EndTime
        FROM TaskTimes
        ORDER BY TaskId, StartTime ASC";
        let records: Vec<TaskTime> = sqlx::query_as(&sql).fetch_all(pool).await?;
        Ok(records)
    }

    async fn retrieve_some(pool: &sqlx::Pool<Sqlite>, uuid: &Uuid) -> Result<Vec<TaskTime>, Error> {
        let sql = "SELECT
            TaskTimeId,
            TaskId,
            StartTime,
            EndTime
        FROM TaskTimes
        WHERE TaskId = ?
        ORDER BY TaskId, StartTime ASC";
        let records: Vec<TaskTime> = sqlx::query_as(&sql).bind(&uuid).fetch_all(pool).await?;
        Ok(records)
    }

    async fn retrieve_one(&mut self, pool: &sqlx::Pool<Sqlite>) -> Result<(), Error> {
        let sql = "SELECT
            TaskTimeId,
            TaskId,
            StartTime,
            EndTime
        FROM TaskTimes
        WHERE TaskTimeId = ?";

        let row: SqliteRow = sqlx::query(&sql)
            .bind(self.task_time_id as i64)
            .fetch_one(pool)
            .await?;
        let temp = TaskTime::from_row(&row)?;

        self.task_time_id = temp.task_time_id;
        self.task_id = temp.task_id;
        self.start_time = temp.start_time;
        self.end_time = temp.end_time;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::project_task::ProjectTask;
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

    use super::TaskTime;
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

    fn make_task_time(task_id: Uuid, time_diff: i64) -> TaskTime {
        let mut task_time = TaskTime::default();
        // Date string  "Sat Aug 10 2024"
        let mut hour = 12 + time_diff;

        task_time.task_time_id = 0;
        task_time.task_id = task_id;

        let dt_str = format!("{}-{}-{} {}:{}:{}", 2024, "08", 10, hour, 30, 30);
        task_time.start_time = NaiveDateTime::parse_from_str(&dt_str, "%Y-%m-%d %H:%M:%S").unwrap();
        hour += time_diff;
        let dt_str = format!("{}-{}-{} {}:{}:{}", 2024, "08", 10, hour, 30, 30);
        task_time.end_time = NaiveDateTime::parse_from_str(&dt_str, "%Y-%m-%d %H:%M:%S").unwrap();

        task_time
    }

    async fn setup() -> Result<DbiDatabase, Error> {
        let config = DbConfig::new("sqlite::memory:");
        let mut db = DbiDatabase::new(config).await?;
        Ok(db)
    }

    #[tokio::test]
    async fn test_insert_one_task_time() -> Result<(), Error> {
        let mut db = setup().await?;
        let project = make_project();
        let num_rows = Project::insert_one(&db.pool, &project).await?;
        assert_eq!(num_rows, 1);

        let task = make_task(project.project_id.clone(), 0);
        let num_rows = ProjectTask::insert_one(&db.pool, &task).await?;
        assert_eq!(num_rows, 1);

        let task_time = make_task_time(task.task_id.clone(), 0);
        let row_id = TaskTime::insert_one(&db.pool, &task_time).await?;
        assert_eq!(row_id, 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_select_one_task_time() -> Result<(), Error> {
        let mut db = setup().await?;
        let project = make_project();
        let num_rows = Project::insert_one(&db.pool, &project).await?;
        assert_eq!(num_rows, 1);

        let task = make_task(project.project_id.clone(), 0);
        let num_rows = ProjectTask::insert_one(&db.pool, &task).await?;
        assert_eq!(num_rows, 1);

        let mut task_time = make_task_time(task.task_id.clone(), 0);
        let row_id = TaskTime::insert_one(&db.pool, &task_time).await?;
        assert_eq!(row_id, 1);
        task_time.task_time_id = row_id;

        let mut actual = TaskTime::default();
        actual.task_time_id = task_time.task_time_id;

        actual.retrieve_one(&db.pool).await?;
        assert_eq!(actual, task_time);

        Ok(())
    }

    #[tokio::test]
    async fn test_select_all_task_times() -> Result<(), Error> {
        let mut db = setup().await?;
        let project = make_project();
        let num_rows = Project::insert_one(&db.pool, &project).await?;
        assert_eq!(num_rows, 1);

        let task = make_task(project.project_id.clone(), 0);
        let num_rows = ProjectTask::insert_one(&db.pool, &task).await?;
        assert_eq!(num_rows, 1);

        let mut expected: Vec<TaskTime> = Vec::with_capacity(3);

        for i in 0..3 {
            let mut task_time = make_task_time(task.task_id.clone(), i);
            let res = TaskTime::insert_one(&db.pool, &task_time).await?;
            assert_eq!(res, (i + 1) as u64);
            task_time.task_time_id = res;
            expected.push(task_time);
        }

        let actual = TaskTime::retrieve_all(&db.pool).await?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_select_some_task_times() -> Result<(), Error> {
        let mut db = setup().await?;

        let project = make_project();
        let num_rows = Project::insert_one(&db.pool, &project).await?;
        assert_eq!(num_rows, 1);

        let task1 = make_task(project.project_id.clone(), 0);
        let num_rows = ProjectTask::insert_one(&db.pool, &task1).await?;
        assert_eq!(num_rows, 1);

        let task2 = make_task(project.project_id.clone(), 1);
        let num_rows = ProjectTask::insert_one(&db.pool, &task2).await?;
        assert_eq!(num_rows, 1);

        let mut expected: Vec<TaskTime> = Vec::with_capacity(3);

        for i in 0..3 {
            let mut task_time = make_task_time(task1.task_id.clone(), i);
            let res = TaskTime::insert_one(&db.pool, &task_time).await?;
            assert_eq!(res, (i + 1) as u64);
            task_time.task_time_id = res;
            expected.push(task_time);
        }

        for i in 3..6 {
            let mut task_time = make_task_time(task2.task_id.clone(), i);
            let res = TaskTime::insert_one(&db.pool, &task_time).await?;
            assert_eq!(res, (i + 1) as u64);
            task_time.task_time_id = res;
        }

        let actual = TaskTime::retrieve_some(&db.pool, &task1.task_id).await?;
        assert_eq!(actual, expected);

        Ok(())
    }
}
