#![allow(unused)]

mod database;
mod model;

use database::query::DbObject;

use model::project;
use model::project::Project;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Row;
use sqlx::Sqlite;
// use sqlx::types::chrono::{NaiveDate, NaiveDateTime};

pub enum DataObject {
    Project(Project),
    #[cfg(test)]
    MyTable(tests::MyTable),
}

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
    pool: Pool<Sqlite>,
}

impl DbiDatabase {
    pub async fn new(config: DbConfig) -> Result<Self, Error> {
        let pool = Pool::connect(&config.url).await?;
        Ok(Self { pool: pool })
    }

    pub async fn create_table(&mut self) -> Result<(), Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS my_table (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                data TEXT NOT NULL,
                created_at DATE NOT NULL
            )",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn do_insert(&mut self, data_object: &mut DataObject) -> Result<u64, Error> {
        match data_object {
            DataObject::Project(_) => todo!(),
            #[cfg(test)]
            DataObject::MyTable(value) => <tests::MyTable>::insert_one(&mut self.pool, value).await,
        }
    }

    pub async fn fetch_one(&mut self, data_object: &mut DataObject) -> Result<(), Error> {
        match data_object {
            DataObject::Project(project) => project.retrieve_one(&self.pool).await,
            #[cfg(test)]
            DataObject::MyTable(table) => table.retrieve_one(&self.pool).await,
        }
        //Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use database::query::DbObject;
    use serde::{Deserialize, Serialize};
    use sqlx::query::Query;
    use sqlx::sqlite::Sqlite;
    use sqlx::sqlite::SqliteArguments;
    use sqlx::sqlite::SqliteRow;
    use sqlx::Column;
    use sqlx::Error;
    use sqlx::FromRow;
    use sqlx::Pool;
    use sqlx::Row;

    #[tokio::test]
    async fn test_database_select() -> Result<(), Error> {
        let config = DbConfig::new("sqlite::memory:");
        let mut db = DbiDatabase::new(config).await?;

        db.create_table().await?;
        let mut inserted = match setup(&mut db).await {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        inserted.id = 1;
        let mut mytable = MyTable::default();
        mytable.id = inserted.id;
        let mut dao = DataObject::MyTable(mytable.clone());
        let result = db.fetch_one(&mut dao).await?;
        if let DataObject::MyTable(mytable) = dao {
            assert_eq!(&mytable, &inserted);
        }

        assert_eq!(result, ());
        Ok(())
    }

    #[tokio::test]
    async fn test_database_insert_query() -> Result<(), Error> {
        let config = DbConfig::new("sqlite::memory:");
        let mut db = DbiDatabase::new(config).await?;
        db.create_table().await?;
        let _inserted = match setup(&mut db).await {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_dbobject_insert_one() -> Result<(), Error> {
        let config = DbConfig::new("sqlite::memory:");
        let mut db = DbiDatabase::new(config).await?;
        db.create_table().await?;
        let mut mt = MyTable {
            id: 0,
            data: "Testing Insert".to_string(),
            created_at: NaiveDate::MAX,
        };
        let mut dao = DataObject::MyTable(mt.clone());
        let result = db.do_insert(&mut dao).await?;
        assert_eq!(result, 1);
        mt.id = 1;
        if let DataObject::MyTable(mytable) = dao {
            assert_eq!(&mytable, &mt);
        }
        Ok(())
    }

    async fn setup(db: &mut DbiDatabase) -> Result<MyTable, Error> {
        let mut my_table = MyTable::default();
        my_table.data = "A setup object".to_string();
        let sql = "INSERT INTO my_table (data, created_at) VALUES (?, ?)";

        let result = sqlx::query(sql)
            .bind(my_table.data.clone())
            .bind(my_table.created_at)
            .execute(&db.pool)
            .await?;

        if result.rows_affected() > 0 {
            let rows = result.rows_affected();
            return Ok(my_table);
        }
        Err(Error::Protocol("Insert returned 0 rows affected".into()))
    }

    #[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize, FromRow)]
    pub struct MyTable {
        id: u64,
        data: String,
        created_at: NaiveDate,
    }

    impl DbObject<Sqlite, MyTable> for MyTable {
        async fn insert_one(pool: &Pool<Sqlite>, dbo: &mut MyTable) -> Result<u64, Error> {
            let query_str = "INSERT INTO my_table (data, created_at) VALUES (?, ?)";

            let result = sqlx::query(query_str)
                .bind(dbo.data.clone())
                .bind(dbo.created_at)
                .execute(pool)
                .await?;
            let insert_count = result.rows_affected();

            if insert_count == 1 {
                let row: SqliteRow = sqlx::query(
                    "SELECT id, data, created_at  FROM my_table WHERE data = ? AND created_at = ?",
                )
                .bind(dbo.data.clone())
                .bind(dbo.created_at)
                .fetch_one(pool)
                .await?;

                let tbl: MyTable = MyTable::from_row(&row)?;
                dbo.id = tbl.id;
                return Ok(tbl.id as u64);
            }

            Err(Error::Protocol("Insert returned 0 rows affected".into()))
        }

        async fn retrieve_all(pool: &Pool<Sqlite>) -> Result<Vec<MyTable>, Error> {
            todo!()
        }

        async fn retrieve_one(&mut self, pool: &Pool<Sqlite>) -> Result<(), Error> {
            let sql = "SELECT id, data, created_at FROM my_table WHERE id = ?";

            let row: SqliteRow = sqlx::query("SELECT * FROM my_table")
                .fetch_one(pool)
                .await?;

            let tbl: MyTable = MyTable::from_row(&row)?;
            eprintln!("Row in table: {:?}", tbl);

            let result: SqliteRow = sqlx::query(sql)
                .bind(self.id as i64)
                .fetch_one(pool)
                .await?;
            self.data = result.try_get("data")?;
            self.created_at = result.try_get("created_at")?;
            Ok(())
        }
    }
}
