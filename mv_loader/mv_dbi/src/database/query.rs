use sqlx::Error;
use sqlx::Pool;


#[allow(dead_code)]
pub trait DbObject<DB, T>
where DB: sqlx::Database
{
    async fn insert_one(pool: &Pool<DB>, dbo: &T) -> Result<u64, Error>;
    async fn retrieve_all(pool: &Pool<DB>) -> Result<Vec<T>, Error>;
    async fn retrieve_one(&mut self, pool: &Pool<DB>) -> Result<(), Error>;
}