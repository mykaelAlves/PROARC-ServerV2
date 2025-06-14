use std::{env, io::Write};
use dotenvy::from_filename;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};


pub mod auth;
pub mod actions; 
pub mod files;

pub const CONFIG_DIR: &str = "config";
pub const ENV_FILE: &str = ".env";
pub const LOG_YAML: &str = "log4rs.yaml";

pub const DEFAULT_BUFFER_SIZE: usize = 1024;

pub const OK: &str = "OK";
pub const NOT_OK: &str = "NOT OK";

pub async fn positive(socket: &mut TcpStream)
{
    match socket.write_all(OK.as_bytes()).await
    {
        Ok(_) => {},
        Err(_) => {
            warn("Failed to send positive");
        }
    }

    socket.flush().await.unwrap();
}

pub async fn negative(socket: &mut TcpStream, msg: &str)
{
    let msg = format!("{NOT_OK}: {msg}");


    match socket.write_all(msg.as_bytes()).await
    {
        Ok(_) => {},
        Err(_) => {
            warn("Failed to send negative");
        }
    }

    socket.flush().await.unwrap();
}

pub async fn read_from_stream_as_utf8(socket: &mut TcpStream) -> String
{
    let mut buffer = [0; DEFAULT_BUFFER_SIZE];

    match socket.read(&mut buffer).await {
        Ok(n) => String::from_utf8_lossy(&buffer[..n])
            .replace('\0', "")
            .to_string(),
        Err(e) => {
            err(&format!("Failed to read from stream: {e}"));
            
            "".to_string()
        }
    }
}

pub fn get_env_var(name: &str) -> String 
{
    env::var(name).expect(format!("{name} must be set").as_str())
}

pub fn load_env()
{
    from_filename(format!("{CONFIG_DIR}/{ENV_FILE}").as_str()).ok();
}

pub fn info(msg: &str)
{
    let time = chrono::Local::now().format("%d/%m/%Y %H:%M:%S");

    println!("[{}INFO{}][{time}] {msg}",
        color::color_bright_blue,
        color::color_reset,
    );
}

pub fn warn(msg: &str)
{
    let time = chrono::Local::now().format("%d/%m/%Y %H:%M:%S");

    let display = format!("[{}WARNING{}][{time}] {msg}",
        color::color_bright_yellow,
        color::color_reset,
    );

    println!("{display}");
    write_to_log(&display);
}

pub fn err(msg: &str)
{
    let time = chrono::Local::now().format("%d/%m/%Y %H:%M:%S");

    let display = format!("[{}ERROR{}][{time}] {msg}",
        color::color_bright_red,
        color::color_reset,
    );

    println!("{display}");
    write_to_log(&display);
    panic!("{display}");
}

fn write_to_log(msg: &str)
{
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open("log/latest.log")
        .unwrap();

    match file.write(msg.as_bytes())
    {
        Ok(_) => {},
        Err(e) =>
        {
            panic!("Failed to write to log: {e}"); // dont use err as it causes recursion
        }
    }
}

pub async fn create_log_file() 
{
    //let time = chrono::Local::now().format("%d-%m-%Y_%H%M%S");

    match tokio::fs::write(&format!("log/latest.log"), "").await
    {
        Ok(_) => {},
        Err(e) => {
            err(&format!("Failed to create log file: {e}"));
        }
    }
}

// https://github.com/eliasjonsson023/inline_colorization/blob/master/src/lib.rs
#[allow(unused)]
mod color 
{
    #[allow(non_upper_case_globals)]
    pub const style_bold: &str = "\x1B[1m";
    // #[allow(non_upper_case_globals)]
    // pub const style_un_bold: &str = "\x1B[21m";
    #[allow(non_upper_case_globals)]
    pub const style_underline: &str = "\x1B[4m";
    // #[allow(non_upper_case_globals)]
    // pub const style_un_underline: &str = "\x1B[24m";
    #[allow(non_upper_case_globals)]
    pub const style_reset: &str = "\x1B[0m";

    #[allow(non_upper_case_globals)]
    pub const color_black: &str = "\x1B[30m";
    #[allow(non_upper_case_globals)]
    pub const color_red: &str = "\x1B[31m";
    #[allow(non_upper_case_globals)]
    pub const color_green: &str = "\x1B[32m";
    #[allow(non_upper_case_globals)]
    pub const color_yellow: &str = "\x1B[33m";
    #[allow(non_upper_case_globals)]
    pub const color_blue: &str = "\x1B[34m";
    #[allow(non_upper_case_globals)]
    pub const color_magenta: &str = "\x1B[35m";
    #[allow(non_upper_case_globals)]
    pub const color_cyan: &str = "\x1B[36m";
    #[allow(non_upper_case_globals)]
    pub const color_white: &str = "\x1B[37m";
    #[allow(non_upper_case_globals)]
    pub const color_bright_black: &str = "\x1B[90m";
    #[allow(non_upper_case_globals)]
    pub const color_bright_red: &str = "\x1B[91m";
    #[allow(non_upper_case_globals)]
    pub const color_bright_green: &str = "\x1B[92m";
    #[allow(non_upper_case_globals)]
    pub const color_bright_yellow: &str = "\x1B[93m";
    #[allow(non_upper_case_globals)]
    pub const color_bright_blue: &str = "\x1B[94m";
    #[allow(non_upper_case_globals)]
    pub const color_bright_magenta: &str = "\x1B[95m";
    #[allow(non_upper_case_globals)]
    pub const color_bright_cyan: &str = "\x1B[96m";
    #[allow(non_upper_case_globals)]
    pub const color_bright_white: &str = "\x1B[97m";
    #[allow(non_upper_case_globals)]
    pub const color_reset: &str = "\x1B[39m";

    #[allow(non_upper_case_globals)]
    pub const bg_black: &str = "\x1B[40m";
    #[allow(non_upper_case_globals)]
    pub const bg_red: &str = "\x1B[41m";
    #[allow(non_upper_case_globals)]
    pub const bg_green: &str = "\x1B[42m";
    #[allow(non_upper_case_globals)]
    pub const bg_yellow: &str = "\x1B[43m";
    #[allow(non_upper_case_globals)]
    pub const bg_blue: &str = "\x1B[44m";
    #[allow(non_upper_case_globals)]
    pub const bg_magenta: &str = "\x1B[45m";
    #[allow(non_upper_case_globals)]
    pub const bg_cyan: &str = "\x1B[46m";
    #[allow(non_upper_case_globals)]
    pub const bg_white: &str = "\x1B[47m";
    #[allow(non_upper_case_globals)]
    pub const bg_bright_black: &str = "\x1B[100m";
    #[allow(non_upper_case_globals)]
    pub const bg_bright_red: &str = "\x1B[101m";
    #[allow(non_upper_case_globals)]
    pub const bg_bright_green: &str = "\x1B[102m";
    #[allow(non_upper_case_globals)]
    pub const bg_bright_yellow: &str = "\x1B[103m";
    #[allow(non_upper_case_globals)]
    pub const bg_bright_blue: &str = "\x1B[104m";
    #[allow(non_upper_case_globals)]
    pub const bg_bright_magenta: &str = "\x1B[105m";
    #[allow(non_upper_case_globals)]
    pub const bg_bright_cyan: &str = "\x1B[106m";
    #[allow(non_upper_case_globals)]
    pub const bg_bright_white: &str = "\x1B[107m";
    #[allow(non_upper_case_globals)]
    pub const bg_reset: &str = "\x1B[49m";
}
