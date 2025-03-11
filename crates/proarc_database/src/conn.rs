use sqlx::{postgres::PgPool, Pool, Postgres};
use std::error::Error;

pub async fn establish_connection(database_url: String) -> Result<Pool<Postgres>, Box<dyn Error>>{
    let pool = PgPool::connect(&database_url).await?;

    Ok(pool)
}