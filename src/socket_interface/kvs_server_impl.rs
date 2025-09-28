use std::str::FromStr;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::key_value_store::key_value_store::KeyValueStore;


/// The main key value store server. Stores a listening address so that
/// it may be able to selectively choose the interfaces it listens on
pub struct KVSServer {
    listen_addr_: String
}


impl KVSServer {
    pub fn new(listening_addr: &str) -> KVSServer {
        return KVSServer {
            listen_addr_: String::from_str(listening_addr).unwrap()
        }
    }

    // TODO: Given that Error is a trait, we should ideally create custom
    // errors that extend it and improve our error reporting system.
    pub async fn main_loop(&self) -> Result<(), Box<dyn std::error::Error>> {
        
        // Spawn a single listener at the appropriate address
        let listener = TcpListener::bind(
            self.listen_addr_.as_str()).await?;
        
        // Create an infinite loop that waits on a connection to the socket.
        // Once a connection is hit, spawn off a handler to this connection
        // that reads the data input to the socket, handles it, and exits
        // gracefully. We do not have persistent sessions as of now.
        loop {
            let (mut socket, addr) = listener.accept().await?;

            tokio::spawn(async move {
                let mut buf = [0u8;1024];
                println!("Received connection from: {:?}", addr);
                let mut closed = false;
                while !closed {
                    match socket.read(&mut buf).await {
                        Ok(n) if n == 0 => closed = true,
                        Ok(n) => {
                            // Example for something that prints here
                            // let val = String::from_utf8_lossy(&buf[..n]);
                            // let trimmed = val.trim();
                            // if trimmed.len() == 0 {
                            //     closed = true;
                            // }
                            // println!("Received: {:?}", trimmed);
                            buf.fill(0);
                            
                        },
                        Err(e) => {
                            eprintln!("Error: {e:?}");
                        }
                    }
                }
                println!("Connection from: {:?} closed", addr);
            });
        }
    }
}