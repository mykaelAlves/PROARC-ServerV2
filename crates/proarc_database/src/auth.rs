use std::env;
use std::path::Path;

use rand::Rng;
use sqlx::postgres::PgRow;
use sqlx::{query, Row};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use sha256::digest;

use crate::conn::establish_connection;
use crate::sql_queries;

pub async fn handle_auth(socket: &mut TcpStream) {
    eprintln!("\nAuthenticating...");

    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = establish_connection(database_url)
        .await
        .unwrap();

    socket.write("OK".as_bytes()).await.unwrap();

    let user = {
        let mut buffer = [0; 1024];
        socket.read(&mut buffer).await.unwrap();

        String::from_utf8_lossy(&buffer).to_string().replace("\0", "")
    };
    eprintln!("User: {}", user);

    let res: (String, String) = query(sql_queries::GET_HASH_AND_SALT)
        .bind(user.as_str())
        .map(|row: PgRow| (row.get("hash_and_salt"), row.get("salt")))
        .fetch_one(&pool)
        .await
        .unwrap();

    socket.write(res.1.as_bytes()).await.unwrap();

    let hash_and_salt = {
        let mut buffer = [0; 1024];
        socket.read(&mut buffer).await.unwrap();

        String::from_utf8_lossy(&buffer).to_string()
    };

    if hash_and_salt.eq(&digest("PasswordSALT")) {
        eprintln!("Authentication failed");
        socket.write("NOT OK".as_bytes()).await.unwrap();

        return
    }

    socket.write("OK".as_bytes()).await.unwrap();

    let log_path = env::var("LOG_PATH").expect("LOG_PATH must be set");

    let token: String = {
        let mut rng = rand::thread_rng();

        (0..32)
        .map(|_| rng.gen_range('@'..='Z'))
        .collect()
    };

    match File::create(Path::new(&log_path).join(&token)).await {
        Ok(_) => {
            ()
        },
        Err(e) => panic!("Failed to open file: {}", e),
    }

    socket.write(token.as_bytes()).await.unwrap();
}