use sqlx::sqlite::SqlitePool;

use crate::err;

async fn connect_to_db() -> SqlitePool
{
    match SqlitePool::connect("sqlite:mydatabase.db").await
    {
        Ok(p) => p,
        Err(e) => 
        {
            err(&format!("Failed to connect to database: {e}")); panic!() // <- no need for this panic bruh
        }
    }
}


