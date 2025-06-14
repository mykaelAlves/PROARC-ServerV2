use crate::get_env_var;

use tokio::net::TcpStream;

const AUTH_TOKEN: &str = "nil";

pub enum RequestType
{
    AUTH,
    VALID,
    INVALID,
    ADM
}

pub fn validate_token(token: &str) -> RequestType
{
    let adm_token = get_env_var("ADM_TOKEN");
    let user_token = get_env_var("USER_TOKEN");

    if token.eq(AUTH_TOKEN) 
    {
        return RequestType::AUTH;
    }
    else if token.eq(&adm_token) 
    {
        return RequestType::ADM;
    }
    else if token.eq(&user_token) 
    {
        return RequestType::VALID;
    }
    else 
    {
        return RequestType::INVALID;
    }
}

pub async fn auth(socket: &mut TcpStream)
{

}
