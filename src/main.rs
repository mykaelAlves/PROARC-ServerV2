use tokio::io::AsyncWriteExt;
use tokio::{fs, task, join};
use tokio::net::{TcpListener, TcpStream};
use tokio::signal;

use std::error::Error as StdError;

use proarc_server_v2::{
    auth, err, get_env_var, info, load_env, read_from_stream_as_utf8, warn
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>>
{
    setup().await;
    server_init().await;

    Ok(())
}

struct ServerGuard;

impl Drop for ServerGuard
{
    fn drop(&mut self)
    {
        info("Server closing, cleaning up...");
        // clean anything that is needed
        // when the server closes
        info("Server closed");
    }
}

async fn setup()
{
    info("Setting up...");

    let create_dir_future = fs::create_dir_all("log/");
    let dotenv_future = task::spawn_blocking(| | load_env());

    let _ = join!(create_dir_future, dotenv_future);

    info("Setup complete");
}

async fn server_init()
{    
    let _guard = ServerGuard; // uses a destructor to clean after closing
    info("Starting server...");

    let addr = get_env_var("SERVER_ADDR");
    let listener = match TcpListener::bind(&addr).await
    {
        Ok(listener) => listener,
        Err(e) =>
        {
            err(&format!("Failed to initiate server ({addr}): {e}"));
            return;
        }
    };

    info("Server running");

    // Main server loop
    loop 
    {
        tokio::select! 
        {
            biased;
            _ = signal::ctrl_c() => {
                warn("Ctrl-C received, initiating shutdown...");
                break;
            }
            result = listener.accept() => 
            {
                match result {
                    Ok((mut socket, addr)) => {
                        info(&format!("Accepted connection from: {}", addr));
                        
                        tokio::spawn(async move 
                        {
                            handle_connection(&mut socket).await;

                            match socket.shutdown().await
                            {
                                Ok(_) => {},
                                Err(e) => 
                                {
                                    warn(
                                        &format!("Failed to shutdown connection from {}: {}", addr, e
                                    ));
                                }
                            }
                            info(&format!("Connection from {} closed", addr));
                        });
                    }
                    Err(e) => {
                        warn(&format!("Failed to accept connection: {}", e));
                        break;
                    }
                }
            }
        }
    }
}

async fn handle_connection(socket: &mut TcpStream)
{
    let token = read_from_stream_as_utf8(socket).await;

    info(&format!("Token: {token}"));

    let request_type = auth::validate_token(&token);

    match request_type 
    {
        auth::RequestType::AUTH =>
        {
            todo!("Auth request")
        }
        auth::RequestType::VALID =>
        {
            todo!("Valid request")
        }
        auth::RequestType::INVALID =>
        {
            todo!("Invalid request")
        }
        auth::RequestType::ADM =>
        {
            todo!("ADM request")
        }
    }
}