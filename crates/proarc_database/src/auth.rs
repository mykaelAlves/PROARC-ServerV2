use std::env;
use std::io::Write;
use std::path::Path;

use rand::Rng;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use sha256::digest;

use crate::conn::establish_connection;
use crate::sql_queries;

pub async fn handle_auth(socket: &mut TcpStream) {
    println!("\nAuthenticating...");

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
    println!("User: {}", user);

    let res = match sqlx::query(sql_queries::GET_HASH_AND_SALT)
        .bind(user.as_str())
        .fetch_one(&pool)
        .await
    {
        Ok(record) => record,
        Err(sqlx::Error::Database(db_err)) if db_err.code() == Some(std::borrow::Cow::Borrowed("22021")) => {
            panic!("Invalid byte sequence for encoding 'UTF8': {}", db_err)
        }
        Err(e) => panic!("Unexpected error: {}", e),
    };

    println!("{:#?}", res);

    socket.write("SALT".as_bytes()).await.unwrap();

    let hash_and_salt = {
        let mut buffer = [0; 1024];
        socket.read(&mut buffer).await.unwrap();

        String::from_utf8_lossy(&buffer).to_string()
    };
    println!("Hash and salt: {}", hash_and_salt);
    println!("Hash and salt: {}", digest("PasswordSALT"));

    if hash_and_salt.eq(&digest("PasswordSALT")) {
        println!("Authentication failed");
        socket.write("NOT OK".as_bytes()).await.unwrap();

        return
    }

    println!("Authentication successful");
    socket.write("OK".as_bytes()).await.unwrap();

    let log_path = env::var("LOG_PATH").expect("LOG_PATH must be set");
    println!("Logging to {}", log_path);

    let token: String = {
        let mut rng = rand::thread_rng();

        (0..32)
        .map(|_| rng.gen_range('@'..='Z'))
        .collect()
    };
    println!("Token: {}", token);
    println!("{:?}", Path::new(&log_path).join(&token));
    match File::create(Path::new(&log_path).join(&token)).await {
        Ok(_) => {
            ()
        },
        Err(e) => panic!("Failed to open file: {}", e),
    }
    socket.write(token.as_bytes()).await.unwrap();
}