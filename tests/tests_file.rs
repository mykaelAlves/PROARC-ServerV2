use std::{fs, env};

fn create_file_in_bucket() {
    dotenvy::dotenv().ok();

    let bucket_path = env::var("FILES_BUCKET")
        .expect("FILES_BUCKET must be set");

    let _ = fs::create_dir_all(&bucket_path);

    let file_path = format!("{}/test.txt", bucket_path);
    let _ = fs::File::create(file_path);
}

#[tokio::test]
async fn upload_file() {

}

#[tokio::test]
async fn download_file() {
    create_file_in_bucket();
}