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
#[sqlx(rename_all = "PascalCase")]
pub struct Project {
    pub project_id: Uuid,
    pub project_name: String,
    pub project_date: NaiveDate,
    pub pay_rate: f64,
    pub project_duration: i32,
    pub total_pay: f64,
}

impl DbObject<Sqlite, Project> for Project {
    async fn insert_one(pool: &sqlx::Pool<Sqlite>, dbo: &mut Project) -> Result<u64, Error> {
        let mut tx = pool.begin().await?;
        let sql = "INSERT INTO Projects (
            ProjectId,
            ProjectName,
            ProjectDate,
            PayRate,
            ProjectDuration,
            TotalPay
        ) VALUES (
            $1, $2, $3, $4, $5, $6
        )";

        let query = sqlx::query(sql)
            .bind::<Uuid>(dbo.project_id)
            .bind::<String>(dbo.project_name.clone())
            .bind::<NaiveDate>(dbo.project_date)
            .bind::<f64>(dbo.pay_rate)
            .bind::<i32>(dbo.project_duration)
            .bind::<f64>(dbo.total_pay)
            .execute(&mut *tx)
            .await?;

        Ok(query.rows_affected())
    }

    async fn retrieve_all(pool: &sqlx::Pool<Sqlite>) -> Result<Vec<Project>, Error> {
        let sql = "SELECT
            ProjectId,
            ProjectName,
            ProjectDate,
            PayRate,
            ProjectDuration,
            TotalPay
        FROM Projects
        ORRDER BY ProjectDate ASC";
        let records: Vec<Project> = sqlx::query_as(&sql).fetch_all(pool).await?;
        Ok(records)
    }

    async fn retrieve_one(&mut self, pool: &sqlx::Pool<Sqlite>) -> Result<(), Error> {
        let sql = "SELECT
            ProjectId,
            ProjectName,
            ProjectDate,
            PayRate,
            ProjectDuration,
            TotalPay
        FROM Projects
        ORRDER BY ProjectDate ASC
        WHERE project_id = ?";

        let row: SqliteRow = sqlx::query(&sql)
            .bind(self.project_id)
            .fetch_one(pool)
            .await?;
        let temp_project = Project::from_row(&row)?;

        self.project_id = temp_project.project_id;
        self.project_name = temp_project.project_name;
        self.project_date = temp_project.project_date;
        self.pay_rate = temp_project.pay_rate;
        self.project_duration = temp_project.project_duration;
        self.total_pay = temp_project.total_pay;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::DbConfig;
    use crate::DbiDatabase;
    use crate::DbObject;
    use chrono::NaiveDate;
    use sqlx::migrate::Migrator;
    use sqlx::Error;
    use uuid::Uuid;

    use super::Project;

    fn make_uuid(value: &String) -> Uuid {
        eprintln!("OID = {}", Uuid::NAMESPACE_OID.to_string());
        let uuid = Uuid::new_v5(&Uuid::NAMESPACE_OID, value.as_bytes());
        uuid
    }

    // Uuid:OID = 6ba7b812-9dad-11d1-80b4-00c04fd430c8
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
        eprintln!("Project: {project:#?}");
        project
    }

    async fn setup() -> Result<DbiDatabase, Error> {
        let config = DbConfig::new("sqlite::memory:");
        let mut db = DbiDatabase::new(config).await?;
        sqlx::migrate!("../migrations").run(&db.pool).await?;
        Ok(db)
    }

    #[tokio::test]
    async fn test_insert_one() -> Result<(), Error> {
        let mut db = setup().await?;
        let mut project = make_project();
        let num_rows = Project::insert_one(&db.pool, &mut project).await?;
        assert_eq!(num_rows, 1);
        Ok(())
    }
}
