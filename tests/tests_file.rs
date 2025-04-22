use std::{fs, env};
use proarc_connection::*;

async fn server() {
    dotenvy::dotenv().ok();

    let server_addr = env::var("SERVER_ADDR")
        .expect("SERVER_ADDR must be set");
    eprintln!("Starting server at {}", server_addr);
    conn::listen(server_addr).await;
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

}

#[tokio::test]
async fn download_file() {
    create_file_in_bucket();
}