use std::{env, fs, io::Read};
use proarc_connection::*;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream, time};

async fn server() {
    dotenvy::dotenv().ok();

    let server_addr = env::var("SERVER_ADDR")
        .expect("SERVER_ADDR must be set");
    eprintln!("Starting server at {}", server_addr);
    conn::listen(server_addr).await;
}

async fn get_response(stream: &mut TcpStream) -> String {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    String::from_utf8_lossy(&buffer).to_string().replace("\0", "")
}

fn create_file_in_bucket() {
    dotenvy::dotenv().ok();

    let bucket_path = env::var("FILES_BUCKET")
        .expect("FILES_BUCKET must be set");

    let _ = fs::create_dir_all(&bucket_path);

    let file_path = format!("{}/test.txt", bucket_path);
    let _ = fs::File::create_new(file_path);
}

#[tokio::test]
async fn upload_file() {
    tokio::spawn(async move {
        server().await;
    });

    time::sleep(time::Duration::from_millis(100)).await;

    dotenvy::dotenv().ok();

    let mut stream = TcpStream::
        connect(env::var("SERVER_ADDR")
        .expect("SERVER_ADDR must be set"))
        .await
        .unwrap();

    let master_token = env::var("MASTER_TOKEN")
        .expect("MASTER_TOKEN must be set");

    // First request and response: go to auth service
    stream.try_write(b"FILE").unwrap();
    time::sleep(time::Duration::from_millis(100)).await;
    stream.try_write(master_token.as_bytes()).unwrap();
    let res: String = get_response(&mut stream).await;
    assert_eq!(res, "OK", 
        "Testing the 1st step of the file upload process: the server should respond with 'OK'");

    // Second request and response
    stream.try_write(b"UPLOAD").unwrap();
    let res: String = get_response(&mut stream).await;
    assert_eq!(res, "OK", 
        "Testing the 2st step of the file upload process: the server should respond with 'OK'");

    // Third request and response
    let files_dir_path = env::var("TEST_FILES_PATH")
        .expect("TEST_FILES_PATH must be set");
    let filename = "colors.png";
    let file_path = format!("{}/{}", files_dir_path, filename);
    stream.try_write(filename.as_bytes()).unwrap();
    let res: String = get_response(&mut stream).await;
    assert_eq!(res, "OK", 
        "Testing the 3st step of the file upload process: the server should respond with 'OK'");

    // Fourth request and response
    let mut file = fs::File::open(file_path).unwrap();
    let mut buffer = [0; 1024];
    loop {
        let n = file.read(&mut buffer).unwrap();

        if n == 0 {
            break;
        }

       stream.try_write(&buffer[..n]).unwrap();
    }
    let res: String = get_response(&mut stream).await;
    assert_eq!(res, "OK", 
        "Testing the 4st step of the file upload process: the server should respond with 'OK'");
}

#[tokio::test]
async fn download_file() {
    create_file_in_bucket();
}