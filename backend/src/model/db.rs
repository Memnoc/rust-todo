use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Result};
use std::time::Duration;

const PG_HOST: &str = "localhost";
const ROOT_DB: &str = "postgres";
const PG_ROOT_USER: &str = "postgres";
const PG_ROOT_PWD: &str = "postgres";

pub type Db = Pool<Postgres>;

pub async fn init_db() -> Result<Db, sqlx::Error> {
    new_db_pool(PG_HOST, ROOT_DB, PG_ROOT_USER, PG_ROOT_PWD, 1).await
}

// PgPoolOptions creates a database pool
// we specify the max connections allowed
// an hardcoded timeout of 500 mls
// and the connections string

async fn new_db_pool(
    host: &str,
    db: &str,
    user: &str,
    pwd: &str,
    max_con: u32,
) -> Result<Db, sqlx::Error> {
    let con_string: String = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
    PgPoolOptions::new()
        .max_connections(max_con)
        .acquire_timeout(Duration::from_millis(500)) //replaced
        .connect(&con_string)
        .await
}

// Unit test
#[cfg(test)]
#[path = "../_tests/model_db.rs"]
mod tests;
