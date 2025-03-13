use std::env;
use std::error::Error;
use proarc_connection::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let server_addr = env::var("SERVER_ADDR").expect("SERVER_ADDR must be set");

    println!("Starting server at {}", server_addr);

    conn::listen(server_addr).await;
    
    Ok(())
}