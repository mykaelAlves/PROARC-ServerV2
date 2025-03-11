use std::env;
use tokio::net::TcpListener;

pub async fn listen(addr: String) {
    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        match listener.accept().await {
            Ok((_socket, addr)) => println!("new client: {addr:?}"),
            Err(e) => println!("couldn't get client: {e:?}"),
        }
    }
}