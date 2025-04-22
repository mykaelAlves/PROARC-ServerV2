use proarc_utils::send_negative;
use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use proarc_database::*;
use proarc_file_transfer::*;

use tokio::time::{sleep, timeout, Duration, Instant};
use tokio::pin;

pub async fn listen(addr: String) {
    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            let idle = sleep(Duration::from_secs(30));
            pin!(idle);

            loop {
                tokio::select! {
                    res = socket.read(&mut buf) => {
                        let n = match res {
                            Ok(0)  => break, 
                            Ok(n)  => n,
                            Err(e) => { eprintln!("Read error: {}", e); break }
                        };
                        idle.as_mut().reset(Instant::now() + Duration::from_secs(30));

                        let opt = String::from_utf8_lossy(&buf[..n]);
                        eprintln!("Option: {}", opt);

                        let token = match timeout(Duration::from_secs(5), proarc_utils::read_message(&mut socket)).await {
                            Ok(t)   => { idle.as_mut().reset(Instant::now() + Duration::from_secs(30)); t },
                            Err(_)      => { eprintln!("Token read timed out"); break; }
                        };

                        if !is_trusted(&token).await {
                            if opt.eq_ignore_ascii_case("AUTH") {
                                auth::handle_auth(&mut socket).await;
                            } else {
                                eprintln!("Invalid token: {}", token);
                                send_negative(&mut socket).await;
                            }
                            break;
                        }

                        match opt.to_uppercase().as_str() {
                            "DB"   => todo!(),
                            "FILE" => file::handle_file(&mut socket).await,
                            _      => eprintln!("Not a valid request"),
                        }
                    }

                    _ = &mut idle => {
                        eprintln!("Connection idle for 30s, shutting down...");
                        break;
                    }
                }
            }
        });
    }
}

async fn is_trusted(_token: &String) -> bool {
    eprintln!("Validating connection by token...");

    if _token.to_uppercase() == "NIL" {
        return false
    }

    true
}