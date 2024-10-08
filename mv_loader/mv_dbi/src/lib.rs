#![allow(unused)]

pub mod database;
pub mod model;
pub mod utils;

use database::query::DbObject;
use model::project_task::ProjectTask;
use model::task_time::TaskTime;
use sqlx::migrate::MigrateDatabase;

use model::project;
use model::project::Project;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Row;
use sqlx::Sqlite;

pub enum DataCollection {
    Projects(Vec<Project>),
    ProjectTasks(Vec<ProjectTask>),
    TaskTimes(Vec<TaskTime>),
    #[cfg(test)]
    MyTables(Vec<tests::MyTable>),
}

pub enum DataObject {
    Project(Project),
    ProjectTask(ProjectTask),
    TaskTime(TaskTime),
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

const MEMORY_DB: &str = "sqlite::memory:";

impl DbiDatabase {
    pub async fn new(config: DbConfig) -> Result<Self, Error> {
        Self::check_and_create_database_file(&config.url).await?;
        let pool = Pool::connect(&config.url).await?;

        let result = sqlx::query("PRAGMA journal_mode = DELETE;")
            .execute(&pool)
            .await?;

        let result = sqlx::query("PRAGMA foreign_keys = ON;")
            .execute(&pool)
            .await?;
        sqlx::migrate!("../migrations").run(&pool).await?;
        Ok(Self { pool })
    }

    async fn check_and_create_database_file(db_url: &str) -> Result<(), sqlx::Error> {
        if !db_url.eq(MEMORY_DB) {
            if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
                Sqlite::create_database(&db_url).await?
            }
        }
        Ok(())
    }

    pub async fn do_insert(&mut self, data_object: &DataObject) -> Result<u64, Error> {
        match data_object {
            DataObject::Project(project) => <Project>::insert_one(&mut self.pool, project).await,
            DataObject::ProjectTask(task) => <ProjectTask>::insert_one(&mut self.pool, task).await,
            DataObject::TaskTime(task_time) => <TaskTime>::insert_one(&mut self.pool, task_time).await,
            #[cfg(test)]
            DataObject::MyTable(value) => <tests::MyTable>::insert_one(&mut self.pool, value).await,
        }
    }

    pub async fn fetch_all(
        &mut self,
        data_object: &DataObject,
    ) -> Result<Box<DataCollection>, Error> {
        let results = match data_object {
            DataObject::Project(project) => Box::new(DataCollection::Projects(
                Project::retrieve_all(&self.pool).await?,
            )),
            DataObject::ProjectTask(project) => Box::new(DataCollection::ProjectTasks(
                ProjectTask::retrieve_all(&self.pool).await?,
            )),
            DataObject::TaskTime(project) => Box::new(DataCollection::TaskTimes(
                TaskTime::retrieve_all(&self.pool).await?,
            )),
            #[cfg(test)]
            DataObject::MyTable(value) => Box::new(DataCollection::MyTables(
                <tests::MyTable>::retrieve_all(&self.pool).await?,
            )),
        };
        // Err(Error::Protocol("Unexpected error".to_string()))
        Ok(results)
    }

    pub async fn fetch_one(&mut self, data_object: &mut DataObject) -> Result<(), Error> {
        match data_object {
            DataObject::Project(project) => project.retrieve_one(&self.pool).await,
            DataObject::ProjectTask(task) => task.retrieve_one(&self.pool).await,
            DataObject::TaskTime(task_time) => task_time.retrieve_one(&self.pool).await,
            #[cfg(test)]
            DataObject::MyTable(table) => table.retrieve_one(&self.pool).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;
    use chrono::NaiveDate;
    use database::query::DbObject;
    use serde::{Deserialize, Serialize};
    use sqlx::pool;
    use sqlx::query::Query;
    use sqlx::sqlite::{SqliteArguments, SqliteRow};
    use sqlx::{Column, Error, FromRow, Pool, Row};

    #[tokio::test]
    async fn test_database_select_one() -> Result<(), Error> {
        let config = DbConfig::new("sqlite::memory:");
        let mut db = DbiDatabase::new(config).await?;

        create_table(&mut db).await?;
        let mut inserted = match setup(&mut db).await {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

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
    async fn test_database_select_all() -> Result<(), Error> {
        let config = DbConfig::new("sqlite::memory:");
        let mut db = DbiDatabase::new(config).await?;
        create_table(&mut db).await?;
        let mut expected: Vec<MyTable> = Vec::with_capacity(3);

        for n in 1..=3 {
            match setup(&mut db).await {
                Ok(value) => expected.push(value),
                Err(error) => return Err(error),
            };
        }

        let mut mytable = MyTable::default();
        let dao = DataObject::MyTable(mytable);
        let result = db.fetch_all(&dao).await?;

        let tables = match *result {
            DataCollection::MyTables(tables) => tables,
            _ => todo!(),
        };

        assert_eq!(tables, expected);
        Ok(())
    }

    #[tokio::test]
    async fn test_dbobject_insert_one() -> Result<(), Error> {
        let config = DbConfig::new("sqlite::memory:");
        let mut db = DbiDatabase::new(config).await?;
        create_table(&mut db).await?;
        let mut mt = MyTable {
            id: 0,
            data: "Testing Insert".to_string(),
            created_at: NaiveDate::MAX,
        };
        let mut dao = DataObject::MyTable(mt.clone());
        let result = db.do_insert(&dao).await?;
        assert_eq!(result, 1);
        mt.id = 1;
        if let DataObject::MyTable(mut mytable) = dao {
            mytable.id = result;
            assert_eq!(&mytable, &mt);
        }
        Ok(())
    }

    async fn setup(db: &mut DbiDatabase) -> Result<MyTable, Error> {
        let mut my_table = MyTable::default();
        my_table.data = "A setup object".to_string();
        let sql = "INSERT INTO MyTable (Data, CreatedAt) VALUES (?, ?)";

        let mut tx = db.pool.begin().await?;
        let result = sqlx::query(sql)
            .bind(my_table.data.clone())
            .bind(my_table.created_at)
            .execute(&mut *tx)
            .await?;

        let row: (i64,) = sqlx::query_as("SELECT last_insert_rowid()")
            .fetch_one(&mut *tx)
            .await?;
        tx.commit().await?;
        my_table.id = row.0 as u64;
        Ok(my_table)
    }

    pub async fn create_table(db: &mut DbiDatabase) -> Result<(), Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS MyTable (
                Id INTEGER PRIMARY KEY AUTOINCREMENT,
                Data TEXT NOT NULL,
                CreatedAt DATE NOT NULL
            )",
        )
        .execute(&db.pool)
        .await?;
        Ok(())
    }

    #[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize, FromRow)]
    #[sqlx(rename_all = "PascalCase")]
    pub struct MyTable {
        id: u64,
        data: String,
        created_at: NaiveDate,
    }

    impl DbObject<Sqlite, MyTable> for MyTable {
        async fn insert_one(pool: &Pool<Sqlite>, dbo: &MyTable) -> Result<u64, Error> {
            let query_str = "INSERT INTO MyTable (Data, CreatedAt) VALUES (?, ?)";
            let mut tx = pool.begin().await?;

            sqlx::query(query_str)
                .bind(dbo.data.clone())
                .bind(dbo.created_at)
                .execute(&mut *tx)
                .await?;

            let row: (i64,) = sqlx::query_as("SELECT last_insert_rowid()")
                .fetch_one(&mut *tx)
                .await?;
            let row_id = row.0 as u64;

            tx.commit().await?;

            Ok(row_id)
        }

        async fn retrieve_all(pool: &Pool<Sqlite>) -> Result<Vec<MyTable>, Error> {
            let sql = "SELECT Id, Data, CreatedAt FROM MyTable ORDER BY Id ASC";
            let records: Vec<MyTable> = sqlx::query_as(&sql).fetch_all(pool).await?;
            Ok(records)
        }

        async fn retrieve_some(
            pool: &Pool<Sqlite>,
            uuid: &uuid::Uuid,
        ) -> Result<Vec<MyTable>, Error> {
            todo!()
        }

        async fn retrieve_one(&mut self, pool: &Pool<Sqlite>) -> Result<(), Error> {
            let sql = "SELECT Id, Data, CreatedAt FROM MyTable WHERE Id = ?";

            let row: SqliteRow = sqlx::query(&sql)
                .bind(self.id as i64)
                .fetch_one(pool)
                .await?;

            let tbl: MyTable = MyTable::from_row(&row)?;
            self.data = tbl.data;
            self.created_at = tbl.created_at;
            Ok(())
        }
    }
}
