use seq_macro::seq;
use sqlx::any::{install_default_drivers, AnyPoolOptions};
use sqlx::{QueryBuilder, Row};

#[tokio::main]
async fn main() {
    install_default_drivers();
    let pool_options = AnyPoolOptions::new();
    let uri = "sqlite:file:foo?mode=memory";
    let pool = pool_options.connect(uri).await.unwrap();
    let mut tx = pool.begin().await.unwrap();

    // create table
    sqlx::query(
        "CREATE TABLE users(
        user_id int PRIMARY KEY,
        user_age int,
        user_weight int NOT NULL
    )",
    )
    .execute(&mut *tx)
    .await
    .unwrap();

    // insert
    let users = (0..).map(|i: i32| (i, i + 1, i + 2));
    let mut query_builder = QueryBuilder::new("INSERT INTO users(user_id, user_age, user_weight)");
    query_builder.push_values(users.into_iter().take(20), |mut b, user| {
        seq!(i in 0..=2 {
            b.push_bind(user.i);
        });
    });
    let query = query_builder.build();
    query.execute(&mut *tx).await.unwrap();

    // fetch
    let ret_fetch = sqlx::query("SELECT * from users")
        .fetch_all(&mut *tx)
        .await
        .unwrap();

    // commit
    tx.commit().await.unwrap();

    assert_eq!(20, ret_fetch.len());
    assert_eq!(Some(0), ret_fetch[0].get::<Option<i32>, _>("user_id"));
    assert_eq!(Some(1), ret_fetch[0].get::<Option<i32>, _>("user_age"));
    assert_eq!(Some(2), ret_fetch[0].get::<Option<i32>, _>("user_weight"));
    assert_eq!(Some(1), ret_fetch[1].get::<Option<i32>, _>("user_id"));
    assert_eq!(Some(2), ret_fetch[1].get::<Option<i32>, _>("user_age"));
    assert_eq!(Some(3), ret_fetch[1].get::<Option<i32>, _>("user_weight"));
}
