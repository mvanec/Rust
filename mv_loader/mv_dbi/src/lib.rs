mod database;
mod model;

#[cfg(test)]
use database::query::DbObject;

use model::project::Project;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Row;
use sqlx::Sqlite;
// use sqlx::types::chrono::{NaiveDate, NaiveDateTime};

use database::query::ToQuery;

pub enum DataObject<'a> {
    Project(&'a Project),
    #[cfg(test)]
    MyTable(&'a tests::MyTable)
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

    pub async fn insert_query(&mut self, item: &impl ToQuery) -> Result<u64, Error> {
        let query = item.to_execute_query();
        let result = query.execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn do_insert(&mut self, data_object: &DataObject<'_>) -> Result<u64, Error> {
        match data_object {
            DataObject::Project(_) => todo!(),
            #[cfg(test)]
            DataObject::MyTable(value) => <tests::MyTable>::insert_one(&self.pool, &value).await,
        }
    }

    pub async fn fetch(&mut self, item: &mut impl ToQuery) -> Result<u64, Error> {
        let query = item.to_fetch_query();
        let result = query.fetch_one(&self.pool).await?;
        let col_count: u64 = result.len() as u64;
        let _ = item.from_row(&result);
        Ok(col_count)
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

        let mut mytable = MyTable {
            id: 0,
            data: String::new(),
            created_at: NaiveDate::MIN,
        };

        let col_count = db.fetch(&mut mytable).await?;

        assert_eq!(col_count, 3);
        inserted.id = mytable.id;
        assert_eq!(&mytable, &inserted);
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
        let mt = MyTable {
            id: 0,
            data: "Testing Insert".to_string(),
            created_at: NaiveDate::MAX,
        };
        let dao = DataObject::MyTable(&mt);
        let result = db.do_insert(&dao).await?;
        assert_eq!(result, 1);
        
        let inserted = MyTable::insert_one(&db.pool, &mt).await?;
        assert_eq!(inserted, 1);
        Ok(())
    }

    async fn setup(db: &mut DbiDatabase) -> Result<MyTable, Error> {
        let my_table = MyTable::default();

        match db.insert_query(&my_table).await {
            Ok(value) => {
                if value <= 0 {
                    return Err(Error::Protocol("Insert returned 0 rows affected".into()));
                }
                Ok(my_table)
            }
            Err(error) => Err(error),
        }
    }

    #[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
    pub struct MyTable {
        id: i64,
        data: String,
        created_at: NaiveDate,
    }

    impl ToQuery for MyTable {
        fn to_execute_query(&self) -> Query<Sqlite, SqliteArguments> {
            let query_str = "INSERT INTO my_table (data, created_at) VALUES (?, ?)";

            let query = sqlx::query(query_str)
                .bind(self.data.clone())
                .bind(self.created_at);
            query
        }

        fn to_fetch_query(&self) -> Query<Sqlite, SqliteArguments> {
            let query_str = "SELECT
                id,
                data,
                created_at
            FROM my_table
            ORDER BY created_at ASC";
            // WHERE project_id = ?";

            let query = sqlx::query(query_str);
            query
        }

        fn from_row(&mut self, row: &SqliteRow) -> Result<(), Error> {
            let cols = row.columns();

            self.id = row.try_get::<i64, _>(cols[0].name())?;
            self.data = row.try_get::<String, _>(cols[1].name())?;
            self.created_at = row.try_get::<NaiveDate, _>(cols[2].name())?;

            Ok(())
        }
    }

    impl DbObject<Sqlite, MyTable> for MyTable {
        async fn insert_one(pool: &Pool<Sqlite>, dbo: &MyTable) -> Result<u64, Error> {
            let query_str = "INSERT INTO my_table (data, created_at) VALUES (?, ?)";

            let result = sqlx::query(query_str)
                .bind(dbo.data.clone())
                .bind(dbo.created_at)
                .execute(pool)
                .await?;
            Ok(result.rows_affected())
        }

        async fn retrieve_one(&self, pool: &Pool<Sqlite>) -> Result<MyTable, Error> {
            todo!()
        }

        async fn retrieve_all(&self, pool: &Pool<Sqlite>) -> Result<Vec<MyTable>, Error> {
            todo!()
        }
    }
}
