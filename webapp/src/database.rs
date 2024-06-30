use std::env::{self, VarError};

pub const ROW_LIMIT: i32 = 12;

pub async fn db() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    sqlx::postgres::PgPool::connect(&get_database_url().unwrap()).await
}

fn get_database_url() -> Result<String, VarError> {
    let db = env::var("POSTGRES_DB").unwrap();
    let user = env::var("POSTGRES_USER").unwrap();
    let password = env::var("POSTGRES_PASSWORD").unwrap();

    Ok(format!(
        "postgres://{}:{}@postgres-service:5432/{}",
        user, password, db
    ))
}
