use PROARC_ServerV2::*;
use tokio::net::{TcpListener, TcpStream};

#[test]
fn get_hash_and_salt() {
    let res: (String, String) = query(proarc_database::sql_queries::GET_HASH_AND_SALT)
        .bind(user.as_str())
        .map(|row: PgRow| (row.get("hash_and_salt"), row.get("salt")))
        .fetch_one(&pool)
        .unwrap();
}

#[test]
fn simple_login() {
    
}