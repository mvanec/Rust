#![allow(unused)]
// models.rs
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::query::Query;
use sqlx::sqlite::Sqlite;
use sqlx::sqlite::SqliteArguments;
use sqlx::sqlite::SqliteRow;
use sqlx::Column;
use sqlx::Error;
use sqlx::FromRow;
use sqlx::Row;
use uuid::Uuid;

use crate::database::query::DbObject;
use crate::DataObject;

#[derive(Debug, Clone, Default, FromRow, PartialEq, Deserialize, Serialize)]
pub struct Project {
    pub project_id: Uuid,
    pub project_name: String,
    pub project_start_date: NaiveDate,
    pub project_end_date: NaiveDate,
    pub pay_rate: f64,
    pub project_duration: i32,
    pub project_total_pay: f64,
}

impl DbObject<Sqlite, Project> for Project {
    async fn insert_one(pool: &sqlx::Pool<Sqlite>, dbo: &mut Project) -> Result<u64, Error> {
        let mut tx = pool.begin().await?;
        let query_str = "INSERT INTO projects (
            project_id,
            project_name,
            project_start_date,
            project_end_date,
            pay_rate,
            project_duration,
            project_total_pay
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7
        )";

        let query = sqlx::query(query_str)
            .bind::<Uuid>(dbo.project_id)
            .bind::<String>(dbo.project_name.clone())
            .bind::<NaiveDate>(dbo.project_start_date)
            .bind::<NaiveDate>(dbo.project_end_date)
            .bind::<f64>(dbo.pay_rate)
            .bind::<i32>(dbo.project_duration)
            .bind::<f64>(dbo.project_total_pay)
            .execute(&mut *tx)
            .await?;

        Ok(query.rows_affected())
    }

    async fn retrieve_all(pool: &sqlx::Pool<Sqlite>) -> Result<Vec<Project>, Error> {
        todo!()
    }

    async fn retrieve_one(&mut self, pool: &sqlx::Pool<Sqlite>) -> Result<(), Error> {
        let query_str = "SELECT
            project_id,
            project_name,
            project_start_date,
            project_end_date,
            pay_rate,
            project_duration,
            project_total_pay
        FROM projects
        ORRDER BY project_start_date ASC,
                  project_end_date ASC
        WHERE project_id = ?";

        let row: SqliteRow = sqlx::query(&query_str)
            .bind(self.project_id)
            .fetch_one(pool)
            .await?;
        let temp_project = Project::from_row(&row)?;

        self.project_id = temp_project.project_id;
        self.project_name = temp_project.project_name;
        self.project_start_date = temp_project.project_start_date;
        self.project_end_date = temp_project.project_end_date;
        self.pay_rate = temp_project.pay_rate;
        self.project_duration = temp_project.project_duration;
        self.project_total_pay = temp_project.project_total_pay;

        Ok(())
    }
}
