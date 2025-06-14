use tokio::{fs, task, join};
use tokio::net::{TcpListener, TcpStream};
use tokio::signal;

use std::error::Error as StdError;

use proarc_server_v2::{
    auth, create_log_file, err, get_env_var, info, load_env, negative, positive,
    read_from_stream_as_utf8, shutdown_socket, warn
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

    create_log_file().await;

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
            _ = signal::ctrl_c() => 
            {
                warn("Ctrl-C received, initiating shutdown...");
                break;
            }
            result = listener.accept() => 
            {
                match result 
                {
                    Ok((mut socket, addr)) => 
                    {
                        info(&format!("Accepted connection from: {}", addr));
                        
                        tokio::spawn(async move 
                        {
                            handle_connection(&mut socket).await;    // <-- main pipeline
                            shutdown_socket(&mut socket).await;
                            
                            info(&format!("Connection from {} closed", addr));
                        });
                    }
                    Err(e) => 
                    {
                        err(&format!("Failed to accept connection: {}", e));
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

    info(&format!("({}) Received token: {token}", socket.peer_addr().unwrap())); // make fn to get addr

    let request_type = auth::validate_token(&token);

    match request_type 
    {
        auth::RequestType::AUTH =>
        {
            positive(socket).await;

            auth::login(socket).await;
        }
        auth::RequestType::VALID =>
        {
            positive(socket).await;

            todo!("Valid request")
        }
        auth::RequestType::INVALID =>
        {
            warn("Invalid token received");

            negative(socket, "Invalid token").await;
        }
        auth::RequestType::ADM =>
        {
            warn("ADM token received");

            positive(socket).await;

            todo!("ADM request")
        }
    }
}