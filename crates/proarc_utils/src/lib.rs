use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const POSITIVE: &[u8] = b"OK";
const NEGATIVE: &[u8] = b"NOT OK";

pub async fn send_positive(socket: &mut TcpStream) {
    socket.write(POSITIVE).await.unwrap();
}

pub async fn send_negative(socket: &mut TcpStream, msg: Option<&str>) {
    let mut msg_ = String::from_utf8_lossy(NEGATIVE).to_string();

    if let Some(m) = msg {
        msg_.push_str("/n");
        msg_.push_str(&m);
    }

    socket.write(msg_.as_bytes()).await.unwrap();
}

pub async fn send_message(socket: &mut TcpStream, message: &str) {
    socket.write(message.as_bytes()).await.unwrap();
}

pub async fn read_message(socket: &mut TcpStream) -> String {
    let mut buffer = [0; 1024];
    socket.read(&mut buffer).await.unwrap();
    String::from_utf8_lossy(&buffer).to_string().replace("\0", "")
}
