use sqlx::query::Query;
use sqlx::sqlite::SqliteArguments;
use sqlx::sqlite::Sqlite;
use sqlx::Error;
use sqlx::sqlite::SqliteRow;

pub trait ToQuery {
    fn to_execute_query(&self) -> Query<Sqlite, SqliteArguments>;
    fn to_fetch_query(&self) -> Query<Sqlite, SqliteArguments>;
    fn from_row(&mut self, row: &SqliteRow) -> Result<(), Error>;
}
