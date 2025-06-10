use tokio::io::AsyncWriteExt;
use tokio::{fs, task, join};
use tokio::net::TcpListener;

use proarc_server_v2::{err, get_env_var, info, load_env, warn};

use std::error::Error as StdError;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[tokio::main]
async fn main() 
{
    setup().await;
    server_init().await;
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
    let mut listener = match TcpListener::bind(&addr).await
    {
        Ok(listener) => listener,
        Err(e) =>
        {
            err(&format!("Failed to initiate server ({addr}): {e}"));
            return;
        }
    };

    info("Server running");

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || 
    {
        warn("Ctrl-C received, initiating shutdown...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Main server loop
    while running.load(Ordering::SeqCst) 
    {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await; //placeholder
    }
}