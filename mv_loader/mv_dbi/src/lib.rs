mod database;
mod model;

use chrono::NaiveDate;
use chrono::NaiveDateTime;
use database::query;
use sqlx::Row;
use sqlx::SqliteConnection;
use sqlx::{Connection, Error};
// use sqlx::types::chrono::{NaiveDate, NaiveDateTime};

use database::query::ToQuery;
use model::project;

#[derive(Debug)]
pub struct DbConfig {
    url: String,
}

impl DbConfig {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_owned(),
        }
    }
}

pub struct DbiDatabase {
    conn: SqliteConnection,
}

impl DbiDatabase {
    pub async fn new(config: DbConfig) -> Result<Self, Error> {
        let conn = SqliteConnection::connect(&config.url).await?;
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

    pub async fn insert_query(&mut self, item: &impl ToQuery) -> Result<u64, Error> {
        let query = item.to_query();
        let result = query.execute(&mut self.conn).await?;
        Ok(result.rows_affected())
    }

    pub async fn insert(&mut self, sql: &str, data: &[DataVariant]) -> Result<u64, Error> {
        let mut query = sqlx::query(sql);
        for param in data {
            match param {
                DataVariant::Int(value) => query = query.bind(value),
                DataVariant::String(value) => query = query.bind(value),
                DataVariant::Float(value) => query = query.bind(value),
                DataVariant::Date(value) => query = query.bind(value),
                DataVariant::DateTime(value) => query = query.bind(value),
            }
        }

        let result = query.execute(&mut self.conn).await?;
        Ok(result.rows_affected())
    }

    pub async fn fetch(&mut self, sql: &str, data: &[DataVariant]) -> Result<u64, Error> {
        let mut query = sqlx::query(sql);
        for param in data {
            match param {
                DataVariant::Int(value) => query = query.bind(value),
                DataVariant::String(value) => query = query.bind(value),
                DataVariant::Float(value) => query = query.bind(value),
                DataVariant::Date(value) => query = query.bind(value),
                DataVariant::DateTime(value) => query = query.bind(value),
            }
        }
        let result = query.fetch_one(&mut self.conn).await?;
        let col_count: u64 = result.len() as u64;
        eprintln!("row = {:?}", result.columns());
        Ok(col_count)
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
    use sqlx::query::Query;
    use sqlx::sqlite::Sqlite;
    use sqlx::sqlite::SqliteArguments;

    #[tokio::test]
    async fn test_database() -> Result<(), Error> {
        let config = DbConfig::new("sqlite::memory:");
        let mut db = DbiDatabase::new(config).await?;

        db.create_table().await?;

        let data = [
            DataVariant::String("Some data".to_string()),
            DataVariant::Date(chrono::Utc::now().date_naive()),
        ];
        let sql = "INSERT INTO my_table (data, created_at) VALUES (?, ?)";
        let rows_affected = db.insert(sql, &data).await?;

        assert_eq!(rows_affected, 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_database_select() -> Result<(), Error> {
        let config = DbConfig::new("sqlite::memory:");
        let mut db = DbiDatabase::new(config).await?;

        db.create_table().await?;
        setup(&mut db).await?;

        let data = [DataVariant::String("Some data".to_string())];
        let sql = "SELECT * FROM my_table WHERE data = ?";
        let rows_affected = db.fetch(sql, &data).await?;

        assert_eq!(rows_affected, 3);

        Ok(())
    }

    #[tokio::test]
    async fn test_database_insert_query() -> Result<(), Error> {
        let config = DbConfig::new("sqlite::memory:");
        let mut db = DbiDatabase::new(config).await?;

        db.create_table().await?;

        let my_table = MyTable {
            id: 0,
            data: "Some text".to_string(),
            created_at: chrono::Utc::now().date_naive()
        };

        let rows_affected = db.insert_query(&my_table).await?;
        assert_eq!(rows_affected, 1);

        Ok(())
    }

    async fn setup(db: &mut DbiDatabase) -> Result<u64, Error> {
        let data = [
            DataVariant::String("Some data".to_string()),
            DataVariant::Date(chrono::Utc::now().date_naive()),
        ];
        let sql = "INSERT INTO my_table (data, created_at) VALUES (?, ?)";
        let rows_affected = db.insert(sql, &data).await?;
        Ok(rows_affected)
    }

    struct MyTable {
        id: i64,
        data: String,
        created_at: NaiveDate,
    }

    impl ToQuery for MyTable {
        fn to_query(&self) -> Query<Sqlite, SqliteArguments> {
            let query_str = "INSERT INTO my_table (data, created_at) VALUES (?, ?)";

            let query = sqlx::query(query_str)
                .bind(self.data.clone())
                .bind(self.created_at);
            query
        }
    }
}
