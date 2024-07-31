use chrono::NaiveDate;
use chrono::NaiveDateTime;
use sqlx::{AnyConnection, Connection, Error};
// use sqlx::types::chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug)]
pub struct DbConfig {
    url: String,
}

impl DbConfig {
    pub fn new(url: &str) -> Self {
        Self { url: url.to_owned() }
    }
}

pub struct Database {
    conn: AnyConnection,
}

impl Database {
    pub async fn new(config: DbConfig) -> Result<Self, Error> {
        sqlx::any::install_default_drivers();
        let conn = sqlx::AnyConnection::connect(&config.url).await?;
        Ok(Self { conn })
    }

    pub async fn create_table(&mut self) -> Result<(), Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS my_table (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                data TEXT NOT NULL,
                created_at DATE NOT NULL
            )",
        )
        .execute(&mut self.conn)
        .await?;
        Ok(())
    }

    pub async fn insert(&mut self, sql: &str, data: &[DataVariant]) -> Result<u64, Error> {
        let mut query = sqlx::query(sql);
        for param in data {
            match param {
                DataVariant::Int(value) => query = query.bind(value),
                DataVariant::String(value) => query = query.bind(value),
                DataVariant::Float(value) => query = query.bind(value),
                DataVariant::Date(value) => {
                    let date_str = value.to_string(); // Convert NaiveDate to String
                    query = query.bind(date_str);
                }
                DataVariant::DateTime(value) => {
                    let datetime_str = value.to_string(); // Convert NaiveDateTime to String
                    query = query.bind(datetime_str);
                }
            }
        }
        eprintln!("Database is {}", self.conn.backend_name());
        let result = query.execute(&mut self.conn).await?;
        Ok(result.rows_affected())
    }
}

pub enum DataVariant {
    Int(i32),
    String(String),
    Float(f32),
    Date(NaiveDate),
    DateTime(NaiveDateTime),
}

#[cfg(test)]
mod tests {
    use super::*;
    //use tokio::test;

    #[tokio::test]
    async fn test_database() -> Result<(), Error> {
        let config = DbConfig::new("sqlite::memory:");
        let mut db = Database::new(config).await?;

        db.create_table().await?;


        let data = [DataVariant::String("Some data".to_string()),
                                      DataVariant::Date(chrono::Utc::now().date_naive())];
        let sql = "INSERT INTO my_table (data, created_at) VALUES (?, ?)";
        let rows_affected = db.insert(sql, &data).await?;

        assert_eq!(rows_affected, 1);

        Ok(())
    }
}
