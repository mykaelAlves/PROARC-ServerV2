use std::env;
use proarc_connection::*;
use tokio::{io::AsyncReadExt, net::TcpStream, time};

async fn server() {
    dotenvy::dotenv().ok();

    let server_addr = env::var("SERVER_ADDR")
        .expect("SERVER_ADDR must be set");
    println!("Starting server at {}", server_addr);
    conn::listen(server_addr).await;
}

async fn get_response(stream: &mut TcpStream) -> String {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    String::from_utf8_lossy(&buffer).to_string().replace("\0", "")
}

#[tokio::test]
async fn simple_login() {
    tokio::spawn(async move {
        server().await;
    }); 

    dotenvy::dotenv().ok();

    let mut stream = TcpStream::
        connect(env::var("SERVER_ADDR")
        .expect("SERVER_ADDR must be set"))
        .await
        .unwrap();

    // First request and response: go to auth service
    stream.try_write(b"AUTH").unwrap();
    time::sleep(time::Duration::from_millis(100)).await;
    stream.try_write(b"nil").unwrap();
    let res: String = get_response(&mut stream).await;
    assert_eq!(res, "OK", 
        "Testing the 1st step of the authentication process: the server should respond with 'OK'");

    // Second request and response: get salt
    stream.try_write(b"ADM").unwrap();
    let salt: String = get_response(&mut stream).await;
    assert_eq!(salt, "SALT", 
        "Testing the 2st step of the authentication process: the server should respond with the salt");

    // Third request and response: send hash and salt
    let mut password: String = env::var("ADM_PASSWORD")
        .expect("PASSWORD must be set");
    password.push_str(&salt);
    stream.try_write(password.as_bytes()).unwrap();
    let res: String = get_response(&mut stream).await;
    assert_eq!(res, "OK", 
        "Testing the 3st step of the authentication process: the server should respond with 'OK' if the password is correct");

    // Fourth request: get token
    let _token = get_response(&mut stream).await;
    assert_ne!(_token, "", 
        "Testing the 4st step of the authentication process: the server should respond with the token");

    println!("Logged sucessfully!");
}