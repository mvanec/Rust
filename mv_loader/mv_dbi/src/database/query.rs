use sqlx::query::Query;
use sqlx::sqlite::SqliteArguments;
use sqlx::sqlite::Sqlite;
use sqlx::Error;
use sqlx::Pool;
use sqlx::sqlite::SqliteRow;

pub trait ToQuery {
    fn to_execute_query(&self) -> Query<Sqlite, SqliteArguments>;
    fn to_fetch_query(&self) -> Query<Sqlite, SqliteArguments>;
    fn from_row(&mut self, row: &SqliteRow) -> Result<(), Error>;
}


#[allow(dead_code)]
pub trait DbObject<DB, T>
where DB: sqlx::Database
{
    async fn insert_one(pool: &Pool<DB>, dbo: &T) -> Result<u64, Error>;
    async fn retrieve_one(&self, pool: &Pool<DB>) -> Result<T, Error>;
    async fn retrieve_all(&self, pool: &Pool<DB>) -> Result<Vec<T>, Error>;
}