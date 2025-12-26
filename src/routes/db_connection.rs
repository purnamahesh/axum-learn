use sqlx::{Pool, Postgres, postgres::PgPool};
use std::error::Error;

pub async fn create_db_connection() -> Result<Pool<Postgres>, Box<dyn Error>> {
    let db_type = "postgres";
    let database = "postgres";
    let username = "postgres";
    let password = "keyoarbcat";
    let host = "localhost";
    let port = "5433";

    let connection_url = format!(
        "{}://{}:{}@{}:{}/{}",
        db_type, username, password, host, port, database
    );

    let conn_pool = PgPool::connect(&connection_url).await?;

    Ok(conn_pool)
}
