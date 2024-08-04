// models.rs
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::query::Query;
use sqlx::sqlite::SqliteArguments;
use sqlx::sqlite::Sqlite;
use sqlx::Error;
use sqlx::sqlite::SqliteRow;

use crate::database::query::ToQuery;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Project {
    pub project_id: Uuid,
    pub project_name: String,
    pub project_start_date: NaiveDate,
    pub project_end_date: NaiveDate,
    pub pay_rate: f64,
    pub project_duration: i32,
    pub project_total_pay: f64,
}

impl ToQuery for Project {
    fn to_execute_query(&self) -> Query<Sqlite, SqliteArguments> {
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
            .bind::<Uuid>(self.project_id)
            .bind::<String>(self.project_name.clone())
            .bind::<NaiveDate>(self.project_start_date)
            .bind::<NaiveDate>(self.project_end_date)
            .bind::<f64>(self.pay_rate)
            .bind::<i32>(self.project_duration)
            .bind::<f64>(self.project_total_pay);
        query
    }

    fn to_fetch_query(&self) -> Query<Sqlite, SqliteArguments> {
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
                  project_end_date ASC";
        // WHERE project_id = ?";

        let query = sqlx::query(query_str);
        query
    }

    fn from_row(&mut self, row: &SqliteRow) -> Result<(), Error> {
        todo!()
    }
}