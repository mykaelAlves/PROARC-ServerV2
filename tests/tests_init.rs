use std::env;

use PROARC_ServerV2::conn;

#[allow(unused_must_use)]
#[test]
fn server_init() {
    dotenvy::dotenv().ok();

    let server_addr = env::var("SERVER_ADDR").expect("SERVER_ADDR must be set");
    println!("Starting server at {}", server_addr);
    conn::listen(server_addr);
}
