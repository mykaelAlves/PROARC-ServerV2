use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use std::{fs, env};

pub async fn handle_file(socket: &mut TcpStream) {
    eprintln!("Connecting to file server...");

    dotenvy::dotenv().ok();

    match check_bucket() {
        Ok(_) => socket.write(b"OK").await.unwrap(),
        Err(e) => {
            socket.write(b"NOT OK").await.unwrap();
            panic!("Error: {}", e);
        },
    };

    let opt: String = {
        let mut buffer = [0; 1024];
        socket.read(&mut buffer).await.unwrap();

        String::from_utf8_lossy(&buffer).to_string().replace("\0", "")
    };

    match opt.as_str() {
        "UPLOAD" => todo!(),
        "DOWNLOAD" => todo!(),
        _ => todo!(),
    }
}

fn check_bucket() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();

    let bucket_path = env::var("FILES_BUCKET")
        .expect("FILES_BUCKET must be set");

    fs::create_dir_all(&bucket_path)
}