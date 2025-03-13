use std::env;

use tokio::net::TcpStream;

use crate::conn::establish_connection;

pub async fn handle_auth(socket: &mut TcpStream) {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = establish_connection(database_url).await.unwrap();

    println!("Authenticating...");
}