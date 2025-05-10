use tokio::{io::AsyncWriteExt, net::TcpStream};
use std::{fs, env};
use proarc_utils::*;

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

    let opt: String = read_message(socket).await;

    match opt.as_str() {
        "UPLOAD" => {
            send_positive(socket).await;
            upload_file(socket).await;
        },
        "DOWNLOAD" => {
            send_positive(socket).await;
            download_file(socket).await;
        },
        _ => {
            send_negative(socket).await;
            panic!("Invalid option: {}", opt);
        },
    }
}

async fn upload_file(socket: &mut TcpStream) {
    let filename = get_filename(socket).await;
}

async fn download_file(socket: &mut TcpStream) {
    let filename = get_filename(socket).await;
}

fn check_bucket() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();

    let bucket_path = env::var("FILES_BUCKET")
        .expect("FILES_BUCKET must be set");

    fs::create_dir_all(&bucket_path)
}

async fn get_filename(socket: &mut TcpStream) -> String {
    let filename: String = read_message(socket).await;

    send_positive(socket).await;

    filename
}