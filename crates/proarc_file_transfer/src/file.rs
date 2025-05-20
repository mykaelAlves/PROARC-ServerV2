//! Still needs to implement download and upload based on Reclamacao's titulo

use std::{io, time::Duration, env, fs::{self}};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    time::timeout,
};
use tracing::{debug, error, instrument};
use proarc_utils::*;

const TIMEOUT_FILE_UPLOAD: Duration = Duration::from_secs(10);

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
            send_negative(socket, None).await;
            panic!("Invalid option: {}", opt);
        },
    }
}

// write to server
async fn upload_file(socket: &mut TcpStream) {
    let directory = match create_dir_if_not_exists(socket).await {
        Ok(dir) => {
            send_positive(socket).await;

            dir
        },
        Err(e) => {
            send_negative(socket, None).await;

            panic!("Error: {}", e);
        }
    };
    
    let (mut file, path) = match get_filename(socket).await {
        Ok(filename) => {
            let path = format!("{}/{}", directory, filename);

            let file = match tokio::fs::File::create_new(&path).await {
                Ok(f) => f,
                Err(e) => {
                    delete_dir_if_empty(&directory).await;

                    send_negative(socket, Some("File already exists")).await;

                    panic!("Error: {}", e);
                }
            };

            send_positive(socket).await;

            (file, path)
        }
        Err(_) => {
            delete_dir_if_empty(&directory).await;

            send_negative(socket, None).await;

            panic!("Invalid filename");
        }
    };

    let contents = match rvc_file_bytes(socket, TIMEOUT_FILE_UPLOAD).await {
        Ok(c) => {
            send_positive(socket).await;

            c
        }
        Err(e) => {
            tokio::fs::remove_file(path).await.unwrap();
            delete_dir_if_empty(&directory).await;
            send_negative(socket, None).await;
            panic!("Error: {}", e);
        }
    };

    match file.write_all(&contents).await {
        Ok(_) => (),
        Err(e) => {
            send_negative(socket, None).await;
            tokio::fs::remove_file(path).await.unwrap();
            panic!("Error: {}", e);
        }
    };
}

async fn delete_dir_if_empty(directory: &str) {
    if tokio::fs::read_dir(directory).await.unwrap().next_entry().await.unwrap().is_none() {
        tokio::fs::remove_dir_all(directory).await.unwrap();
    }
}

async fn get_filename(socket: &mut TcpStream) -> Result<String, ()> {
    let filename = read_message(socket).await;

    if filename.is_empty() {
        return Err(())
    }

    if filename.contains("/") || filename.contains("\\") {
        return Err(())
    }

    Ok(filename)
}

async fn create_dir_if_not_exists(socket: &mut TcpStream) -> Result<String, std::io::Error> {
    let directory = {
        let recl_titulo = read_message(socket).await;

        format!(
            "{}/{}",
            env::var("FILES_BUCKET").expect("FILES_BUCKET must be set"),
            recl_titulo)
    };

    match tokio::fs::create_dir(&directory).await {
        Ok(_) => (),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => panic!("Error: {}", e),
        },
    };

    Ok(directory)
}

// send to client
async fn download_file(socket: &mut TcpStream) {
    
}

// async fn send_file_contents(socket: &mut TcpStream, contents: Vec<u8>) {
//     for chunk in contents.chunks(1024) {
//         socket.write(chunk).await.unwrap();
//     }
// }

// async fn lookfor(filename: &str) -> std::io::Result<Vec<u8>> {
//     dotenvy::dotenv().ok();

//     let bucket_path = env::var("FILES_BUCKET")
//         .expect("FILES_BUCKET must be set");

//     let file_path = format!("{}/{}", bucket_path, filename);

//     read(file_path).await
// }

fn check_bucket() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();

    let bucket_path = env::var("FILES_BUCKET")
        .expect("FILES_BUCKET must be set");

    fs::create_dir_all(&bucket_path)
}

// // wrap return in Result and panic only in the main pipeline
// async fn split_filename(filename: String, socket: &mut TcpStream) -> (String, String) {
//     let (name, ext) = match filename
//         .split_once('.') {
//             Some((name, ext)) => (name, ext),
//             None => {
//                 send_negative(socket).await;
//                 panic!("Invalid file format: {}", filename);
//             }
//     };

//     (name.to_string(), ext.to_string())
// }

#[instrument(skip(socket))]
pub async fn rvc_file_bytes(
    socket: &mut TcpStream,
    read_timeout: Duration,
) -> io::Result<Vec<u8>> {
    let mut reader = BufReader::new(socket);
    let mut contents = Vec::new();
    
    match timeout(read_timeout, reader.read_to_end(&mut contents)).await {
        Err(_) => {
            error!("read_to_end timed out after {:?}", read_timeout);
            Err(io::Error::new(io::ErrorKind::TimedOut, "read timed out"))
        }
        Ok(Err(e)) => {
            error!("I/O error during read: {}", e);
            Err(e)
        }
        Ok(Ok(bytes_read)) => {
            debug!("Completed read_to_end: {} bytes", bytes_read);
            Ok(contents)
        }
    }
}

// fn build_file(name: String, ext: String, contents: Vec<u8>) -> Result<(), std::io::Error> {
//     dotenvy::dotenv().ok();
//     let bucket_path = env::var("FILES_BUCKET")
//         .expect("FILES_BUCKET must be set");
//     std::fs::write(format!("{}/{}.{}", bucket_path, name, ext), contents) 
// }