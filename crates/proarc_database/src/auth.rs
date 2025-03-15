use std::env;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use sha256::digest;

use crate::conn::establish_connection;
use crate::sql_queries;

pub async fn handle_auth(socket: &mut TcpStream) {
    dotenvy::dotenv().ok();

    println!("Authenticating...");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = establish_connection(database_url)
        .await
        .unwrap();

    socket.write("OK".as_bytes()).await.unwrap();

    let user = {
        let mut buffer = [0; 1024];
        socket.read(&mut buffer).await.unwrap();

        String::from_utf8_lossy(&buffer).to_string()
    };
    println!("User: {}", user);

    socket.write("SALT".as_bytes()).await.unwrap();

    let hash_and_salt = {
        let mut buffer = [0; 1024];
        socket.read(&mut buffer).await.unwrap();

        String::from_utf8_lossy(&buffer).to_string()
    };
    println!("Hash and salt: {}", hash_and_salt);
    println!("Hash and salt: {}", digest("PasswordSALT"));

    // let res = sqlx::query(sql_queries::GET_HASH_AND_SALT)
    //     .fetch_one(&pool)
    //     .await
    //     .unwrap();

    // println!("{:#?}", res);
}