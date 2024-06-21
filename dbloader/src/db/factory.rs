use async_trait::async_trait;
use sqlx::{Error, Pool, Sqlite, Postgres, MySql};
use std::error::Error as StdError;

// Define an enum to hold connection parameters for different databases
pub enum DbParam<'a> {
    Sqlite(&'a str),
    Postgres(&'a str),
    MySql(&'a str),
}

// Define a trait for database pool operations
#[async_trait]
pub trait DatabasePool {
    async fn execute(&self, query: &str) -> Result<(), Error>;
}

// Implement DatabasePool trait for each database type
#[async_trait]
impl DatabasePool for Pool<Sqlite> {
    async fn execute(&self, query: &str) -> Result<(), Error> {
        sqlx::query(query).execute(self).await?;
        Ok(())
    }
}

#[async_trait]
impl DatabasePool for Pool<Postgres> {
    async fn execute(&self, query: &str) -> Result<(), Error> {
        sqlx::query(query).execute(self).await?;
        Ok(())
    }
}

#[async_trait]
impl DatabasePool for Pool<MySql> {
    async fn execute(&self, query: &str) -> Result<(), Error> {
        sqlx::query(query).execute(self).await?;
        Ok(())
    }
}

// Factory function to create database pool based on DbParam
pub async fn create_pool(param: DbParam<'_>) -> Result<Box<dyn DatabasePool>, Error> {
    match param {
        DbParam::Sqlite(conn_str) => {
            let pool = Pool::<Sqlite>::connect(conn_str).await?;
            Ok(Box::new(pool))
        }
        DbParam::Postgres(conn_str) => {
            let pool = Pool::<Postgres>::connect(conn_str).await?;
            Ok(Box::new(pool))
        }
        DbParam::MySql(conn_str) => {
            let pool = Pool::<MySql>::connect(conn_str).await?;
            Ok(Box::new(pool))
        }
    }
}

// // Example function to demonstrate using the database pool
// async fn use_database(pool: Box<dyn DatabasePool>) -> Result<(), Error> {
//     pool.execute("SELECT * FROM my_table").await?;
//     Ok(())
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn StdError>> {
//     // Example usage
//     let param = DbParam::Sqlite("sqlite://mydatabase.db");
//     let pool = create_pool(param).await?;

//     use_database(pool).await?;

//     Ok(())
// }
