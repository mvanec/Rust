use sqlx::query::Query;
use sqlx::sqlite::SqliteArguments;
use sqlx::sqlite::Sqlite;

pub trait ToQuery {

    fn to_query(&self) -> Query<Sqlite, SqliteArguments>;
}
