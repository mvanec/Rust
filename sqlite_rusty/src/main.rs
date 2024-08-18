/*
[dependencies]
sqlx = { version = "0.5.9", features = [ "runtime-async-std-native-tls", "sqlite" ] }
async-std = { version = "1.6", features = [ "attributes" ] }
futures = "0.3.18"
*/

use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Sqlite, SqlitePool};
use std::result::Result;

async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(&db_url).await?;
    let qry =
    "PRAGMA foreign_keys = ON ;
    CREATE TABLE IF NOT EXISTS settings
        (
            settings_id             INTEGER PRIMARY KEY NOT NULL,
            description             TEXT                NOT NULL,
            created_on              DATETIME DEFAULT (datetime('now','localtime')),
            updated_on              DATETIME DEFAULT (datetime('now','localtime')),
            done                    BOOLEAN             NOT NULL DEFAULT 0
        );
    CREATE TABLE IF NOT EXISTS project
        (
            project_id                   INTEGER PRIMARY KEY AUTOINCREMENT,
            product_name                 TEXT ,
            created_on                   DATETIME DEFAULT (datetime('now','localtime')),
            updated_on                   DATETIME DEFAULT (datetime('now','localtime')),
            img_directory                TEXT NOT NULL,
            out_directory                TEXT NOT NULL,
            status                       TEXT NOT NULL,
            settings_id                  INTEGER  NOT NULL DEFAULT 1,
            FOREIGN KEY (settings_id)    REFERENCES settings (settings_id) ON UPDATE SET NULL ON DELETE SET NULL
        );";
    let result = sqlx::query(&qry).execute(&pool).await;
    pool.close().await;
    return result;
}

const MEMORY_DB: &str = "sqlite::memory:";

#[async_std::main]
async fn main() {
    let db_url = String::from("sqlite://sqlite.db");

    if !db_url.eq(MEMORY_DB) {
        if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            Sqlite::create_database(&db_url).await.unwrap();
            match create_schema(&db_url).await {
                Ok(_) => println!("Database created Sucessfully"),
                Err(e) => panic!("{}", e),
            }
        }
    }
    let instances = SqlitePool::connect(&db_url).await.unwrap();
    let qry = "INSERT INTO settings (description) VALUES($1)";
    let result = sqlx::query(&qry).bind("testing").execute(&instances).await;

    instances.close().await;

    println!("{:?}", result);
}
