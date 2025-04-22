use std::env;
use proarc_database::conn::establish_connection;

#[tokio::test]
async fn try_establish_connection() {
    dotenvy::dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let _pool = establish_connection(database_url)
        .await
        .unwrap();
}