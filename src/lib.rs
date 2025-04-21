/*
.env file is needed with the following content:

SERVER_ADDR = "<ip>:<port>"
DATABASE_URL = "<database url>"
LOG_PATH = "<path>"
ADM_PASSWORD = "<password>"
FILES_BUCKET = "<path>"
*/

#![allow(non_snake_case)]

pub use proarc_connection::*;