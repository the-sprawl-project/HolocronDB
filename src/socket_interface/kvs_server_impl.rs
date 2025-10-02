use std::str::FromStr;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::key_value_store::key_value_store::KeyValueStore;

use super::decode_utils::{parse_generic_request, parse_ping_request};
use crate::proto::{GenericRequest, PingRequest};


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
        // gracefully.
        loop {
            let (mut socket, addr) = listener.accept().await?;

            tokio::spawn(async move {
                let (mut reader, mut writer) = socket.into_split();
                let mut buf = [0u8;1024];
                println!("Received connection from: {:?}", addr);
                let mut closed = false;
                while !closed {
                    match reader.read(&mut buf).await {
                        Ok(n) if n == 0 => {closed = true},
                        Ok(n) => {
                            // Parse a generic request from the socket
                            let received_req: GenericRequest;
                            match parse_generic_request(&buf[..n]) {
                                Ok(v) => { received_req = v; },
                                Err(e) => {
                                    eprintln!("Parse error: {:?}", e);
                                    return;
                                }
                            };
                            let payload = received_req.payload;
                            let ping_request: PingRequest;
                            
                            match parse_ping_request(&payload) {
                                Ok(v) => { ping_request = v; },
                                Err(e) => {
                                    eprintln!("Parse error: {:?}", e);
                                    return;
                                }
                            };
                            let message: String = ping_request.ping_message;
                            println!("Received ping: {:?}", message);
                            if let Err(e) = writer.write_all(message.as_bytes()).await {
                                eprintln!("Could not write back to socket: {:?}", e);
                            }
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