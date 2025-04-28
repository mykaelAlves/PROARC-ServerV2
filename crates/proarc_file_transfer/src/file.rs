use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
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
    let (name, ext) = split_filename(filename, socket).await;

    send_positive(socket).await;

    let file_contents = rvc_file_bytes(socket).await;
    match build_file(name, ext, file_contents) {
        Ok(_) => send_positive(socket).await,
        Err(e) => {
            send_negative(socket).await;
            panic!("Could not upload file: {}", e);
        },
    }
}

async fn download_file(socket: &mut TcpStream) {
    let filename = get_filename(socket).await;
    let (name, ext) = split_filename(filename, socket).await;

    send_positive(socket).await;
}

fn check_bucket() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();

    let bucket_path = env::var("FILES_BUCKET")
        .expect("FILES_BUCKET must be set");

    fs::create_dir_all(&bucket_path)
}

async fn get_filename(socket: &mut TcpStream) -> String {
    let filename: String = read_message(socket).await;

    filename
}

// wrap return in Result and panic only in the main pipeline
async fn split_filename(filename: String, socket: &mut TcpStream) -> (String, String) {
    let (name, ext) = match filename
        .split_once('.') {
            Some((name, ext)) => (name, ext),
            None => {
                send_negative(socket).await;
                panic!("Invalid file format: {}", filename);
            }
    };

    (name.to_string(), ext.to_string())
}

async fn rvc_file_bytes(socket: &mut TcpStream) -> Vec<u8> {
    let mut contents: Vec<u8> = Vec::new();
    
    loop {
        let mut buffer = [0; 1024];
        let n = match tokio::time::timeout(tokio::time::Duration::from_secs(5), 
        socket.read(&mut buffer)).await.unwrap() {
            Ok(n) => n,
            Err(_) => {
                panic!("Read timed out");
            }
        };

        eprintln!("Read {} bytes", n);

        if n == 0 || n < 1024 {
            break;
        }

        contents.extend_from_slice(&buffer);
    }

    contents
}

fn build_file(name: String, ext: String, contents: Vec<u8>) -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();
    let bucket_path = env::var("FILES_BUCKET")
        .expect("FILES_BUCKET must be set");
    std::fs::write(format!("{}/{}.{}", bucket_path, name, ext), contents) 
}