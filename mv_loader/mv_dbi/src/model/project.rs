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
use sqlx::Row;
use uuid::Uuid;

use crate::database::query::DbObject;
use crate::DataObject;

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Project {
    pub project_id: Uuid,
    pub project_name: String,
    pub project_start_date: NaiveDate,
    pub project_end_date: NaiveDate,
    pub pay_rate: f64,
    pub project_duration: i32,
    pub project_total_pay: f64,
}

impl DbObject<Sqlite, Project> for Project
{
    async fn insert_one(pool: &sqlx::Pool<Sqlite>, dbo: &Project) -> Result<u64, Error> {
        todo!()
    }

    async fn retrieve_all(pool: &sqlx::Pool<Sqlite>) -> Result<Vec<Project>, Error> {
        todo!()
    }

    async fn retrieve_one(&mut self, pool: &sqlx::Pool<Sqlite>) -> Result<(), Error> {
        Ok(())
    }
}

// impl ToQuery for Project {
//     fn to_execute_query(&self) -> Query<Sqlite, SqliteArguments> {
//         let query_str = "INSERT INTO projects (
//             project_id,
//             project_name,
//             project_start_date,
//             project_end_date,
//             pay_rate,
//             project_duration,
//             project_total_pay
//         ) VALUES (
//             $1, $2, $3, $4, $5, $6, $7
//         )";

//         let query = sqlx::query(query_str)
//             .bind::<Uuid>(self.project_id)
//             .bind::<String>(self.project_name.clone())
//             .bind::<NaiveDate>(self.project_start_date)
//             .bind::<NaiveDate>(self.project_end_date)
//             .bind::<f64>(self.pay_rate)
//             .bind::<i32>(self.project_duration)
//             .bind::<f64>(self.project_total_pay);
//         query
//     }

//     fn to_fetch_query(&self) -> Query<Sqlite, SqliteArguments> {
//         let query_str = "SELECT
//             project_id,
//             project_name,
//             project_start_date,
//             project_end_date,
//             pay_rate,
//             project_duration,
//             project_total_pay
//         FROM projects
//         ORRDER BY project_start_date ASC,
//                   project_end_date ASC";
//         // WHERE project_id = ?";

//         let query = sqlx::query(query_str);
//         query
//     }

//     fn from_row(&mut self, row: &SqliteRow) -> Result<(), Error> {
//         let cols = row.columns();

//         if cols.len() < 7 {
//             let message = format!("Not enough columns in result: {:?}", cols);
//             return Err(Error::Protocol(message.into()));
//         }

//         self.project_id = row.try_get::<Uuid, _>(cols[0].name())?;
//         self.project_name = row.try_get::<String, _>(cols[1].name())?;
//         self.project_start_date = row.try_get::<NaiveDate, _>(cols[2].name())?;
//         self.project_end_date = row.try_get::<NaiveDate, _>(cols[3].name())?;
//         self.pay_rate = row.try_get::<f64, _>(cols[4].name())?;
//         self.project_duration = row.try_get::<i32, _>(cols[5].name())?;
//         self.project_total_pay = row.try_get::<f64, _>(cols[6].name())?;

//         Ok(())
//     }
// }
