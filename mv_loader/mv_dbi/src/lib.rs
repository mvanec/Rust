use sqlx::{Any, Connection, Error, Executor, Row};

#[derive(Debug)]
pub struct DbConfig {
    url: String,
}

impl DbConfig {
    pub fn new(url: &str) -> Self {
        Self { url: url.to_owned() }
    }
}

pub struct Database<C> {
    conn: C,
}

impl<C> Database<C>
where
    C: Executor<'static, Database = Self>,
    C: Any,
{
    pub async fn new(config: DbConfig) -> Result<Self, Error> {
        let conn = sqlx::Any::connect(&config.url).await?;
        Ok(Self { conn })
    }

    pub async fn create_table(&self) -> Result<(), Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS my_table (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                data TEXT NOT NULL,
                created_at DATE NOT NULL
            )",
        )
        .execute(&mut self.conn)
        .await?;
        Ok(())
    }

    pub async fn insert(&self, sql: &str, data: &[(&dyn ToSql, &dyn ToSql)]) -> Result<u64, Error> {
        let mut query = sqlx::query(sql);
        for (param, _) in data {
            query = query.bind(param);
        }
        query.execute(&mut self.conn).await?.rows_affected()
    }
}

trait ToSql {
    fn to_sql(&self) -> Result<String, Error>;
}

impl ToSql for str {
    fn to_sql(&self) -> Result<String, Error> {
        Ok(format!("'{self}'"))
    }
}

impl ToSql for &str {
    fn to_sql(&self) -> Result<String, Error> {
        Ok(format!("'{self}'"))
    }
}

impl<T> ToSql for Option<T>
where
    T: ToSql,
{
    fn to_sql(&self) -> Result<String, Error> {
        if let Some(val) = self {
            val.to_sql()
        } else {
            Ok("NULL".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_database() -> Result<(), Error> {
        let config = DbConfig::new("sqlite::memory:");
        let mut db = Database::new(config).await?;

        db.create_table().await?;

        let data = ("Some data", chrono::Utc::now().date());
        let sql = "INSERT INTO my_table (data, created_at) VALUES (?, ?)";
        let rows_affected = db.insert(sql, &[&data.0, &data.1]).await?;

        assert_eq!(rows_affected, 1);

        Ok(())
    }
}
