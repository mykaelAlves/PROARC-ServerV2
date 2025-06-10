use crate::get_env_var;
use std::env;

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

    if token == AUTH_TOKEN 
    {
        return RequestType::AUTH;
    }
    else if token == adm_token 
    {
        return RequestType::ADM;
    }
    else if token == user_token 
    {
        return RequestType::VALID;
    }
    else 
    {
        return RequestType::INVALID;
    }
}