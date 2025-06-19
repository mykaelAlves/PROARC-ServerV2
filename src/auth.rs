use crate::{get_env_var, read_to_string_from_stream};

use tokio::net::TcpStream;

const AUTH_TOKEN: &str = "nil";

pub enum RequestType {
    AUTH,
    VALID,
    INVALID,
    ADM,
}

pub fn validate_token(token: &str) -> RequestType {
    let adm_token = get_env_var("ADM_TOKEN");
    let user_token = get_env_var("USER_TOKEN");

    if token.eq(AUTH_TOKEN) {
        RequestType::AUTH
    } else if token.eq(&adm_token) {
        RequestType::ADM
    } else if token.eq(&user_token) {
        RequestType::VALID
    } else {
        RequestType::INVALID
    }
}

pub async fn login(socket: &mut TcpStream) {
    send_salt(socket).await;
    let pwd = receive_pwd(socket).await;

    if validate_pwd(socket, &pwd).await {
        send_token(socket).await;
    }
}

async fn send_salt(socket: &mut TcpStream) { todo!() }
async fn receive_pwd(socket: &mut TcpStream) -> String { todo!() }
async fn validate_pwd(socket: &mut TcpStream, pwd: &str) -> bool { todo!() }
async fn send_token(socket: &mut TcpStream) { todo!() }


