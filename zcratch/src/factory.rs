use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, Database, Error, Pool, Postgres, Sqlite};
use std::error::Error as StdError;

pub trait DatabaseTraits {
    type DB: Database;
    fn new(uri: &'static str) -> Self;

    fn uri(&self) -> &str;
    async fn connect(&self) -> Result<Pool<Self::DB>, Error>;
}

pub struct PgDatabaseTraits { uri: &'static str }

impl DatabaseTraits for PgDatabaseTraits {
    type DB = Postgres;

     // `Self` is the implementor type: `Sheep`.
     fn new(uri: &'static str) -> PgDatabaseTraits {
        PgDatabaseTraits {uri: uri }
    }

    fn uri(&self) -> &str {
        self.uri
    }

    async fn connect(&self) -> Result<Pool<Self::DB>, Error> {
        let conn: Pool<Self::DB> = PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.uri())
            .await?
            .into();
        Ok(conn)
    }

}
