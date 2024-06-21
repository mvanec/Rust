use async_trait::async_trait;
use sqlx::{postgres::PgDatabaseError, PgPool};
use std::{fmt, io};

#[async_trait(?Send)]
pub trait ModelTrait {
    async fn create(&self, pool: &PgPool) -> Result<(), sqlx::Error>;
    #[allow(dead_code)]
    async fn delete(&self, pool: &PgPool) -> Result<(), sqlx::Error>;
}

// Custom error type that wraps sqlx::Error
#[derive(Debug)]
pub struct DatabaseError {
    pub error: sqlx::Error,
}

impl DatabaseError {
    pub fn new(error: sqlx::Error) -> Self {
        DatabaseError { error }
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.error {
            sqlx::Error::Database(e) => {
                writeln!(f, "Database Error {{")?;
                writeln!(f, "    Code       : {:?}", e.as_ref().code())?;
                writeln!(f, "    Kind       : {:?}", e.as_ref().kind())?;
                writeln!(f, "    Message    : {}", e.as_ref().message())?;
                writeln!(f, "    Table      : {:?}", e.as_ref().table())?;
                writeln!(f, "    Constraint : {:?}", e.as_ref().constraint())?;
                writeln!(f, "    Unique Viol: {}", e.as_ref().is_unique_violation())?;
                writeln!(f, "    FK Viol.   : {}", e.as_ref().is_foreign_key_violation())?;
                writeln!(f, "    Check Viol : {}", e.as_ref().is_check_violation())?;

                let pgopt: Option<&PgDatabaseError> = e.try_downcast_ref::<PgDatabaseError>();
                match pgopt {
                    Some(pgerr) => {
                        writeln!(f, "    Details    : {:?}", pgerr.detail())?;
                        writeln!(f, "    Hint       : {:?}", pgerr.hint())?;
                        writeln!(f, "    Column     : {:?}", pgerr.column())?;
                    },
                    None => eprintln!("Found unknown error type"),
                }
                writeln!(f, "}}")?;
            }
            _ => {
                writeln!(f, "{:?}", self.error)?;
            }
        }
        Ok(())
    }
}

// Usage
pub async fn load_from_csv<T, F>(path: &str, pool: &PgPool, f: F) -> Result<(), io::Error>
where
    F: Fn(Vec<String>) -> T,
    T: ModelTrait + Send + 'static,
{
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;
    for result in rdr.records() {
        let record = result?;
        let record: Vec<String> = record.into_iter().map(|s| s.to_string()).collect();
        let model = f(record);
        model.create(pool).await.map_err(|e| {
            let error = DatabaseError::new(e);
            io::Error::new(io::ErrorKind::Other, format!("{}", error))
        })?;
    }
    Ok(())
}
