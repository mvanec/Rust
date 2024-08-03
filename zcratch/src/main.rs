#![allow(unused_variables)]
#![allow(unused_imports)]
mod factory;

use std::str::FromStr;

use seq_macro::seq;
use uuid;
// use chrono::NaiveDate;
use chrono::naive::NaiveDate;
use sqlx::any::{install_default_drivers, AnyPoolOptions};
use sqlx::{Pool, Postgres, QueryBuilder, Row};
use crate::factory::{DatabaseTraits, PgDatabaseTraits};

#[tokio::main]
async fn main() {
    install_default_drivers();
    let pg_uri = "postgres://u_timekeeper:postgres@192.168.254.12/timekeeper_app";
    let uri = "sqlite:file:foo?mode=memory";

    let db = <PgDatabaseTraits as DatabaseTraits>::new(&pg_uri);
    let pgpool: Pool<Postgres> = db.connect().await.unwrap();
    let mut tx = pgpool.begin().await.unwrap();

    // fetch
    let ret_fetch = sqlx::query("SELECT ProjectId, ProjectName from Projects")
        .fetch_all(&mut *tx)
        .await
        .unwrap();

    // commit
    tx.commit().await.unwrap();

    for rec in &ret_fetch {
        let prid = rec.get::<Option<uuid::Uuid>, _>("projectid");
        let name = rec.get::<Option<String>, _>("projectname");
        println!("{} | {}", &prid.unwrap(), &name.unwrap());
    }
    try_deserialize();
}

fn try_deserialize() {
    let start_date = "2023-08-01";
    let n_date: NaiveDate = NaiveDate::from_str(&start_date).unwrap();
    println!("Parsed date = {}", n_date);
}