use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use proarc_database::*;
use proarc_file_transfer::*;

pub async fn listen(addr: String) {
    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                let rcv = String::from_utf8_lossy(&buf[0..n]);

                println!("Received: {}", rcv);

                match rcv.to_uppercase().as_str() {
                    "AUTH" => auth::handle_auth(&mut socket).await,
                    "DB" => todo!(),
                    "FILE" => file::handle_file(&mut socket).await,
                    _ => println!("Not a valid request"),
                }
            }
        });
    }
}