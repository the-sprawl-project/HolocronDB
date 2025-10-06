use std::str::FromStr;
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::key_value_store::key_value_store::KeyValueStore;

use super::decode_utils::*;
use crate::proto::{CreateKvPairReq, GenericRequest, KeyValuePair, PingRequest, ReqType};


/// The main key value store server. Stores a listening address so that
/// it may be able to selectively choose the interfaces it listens on
pub struct KVSServer {
    listen_addr_: String,
    kvs_access_: RwLock<KeyValueStore>
}


impl KVSServer {
    pub fn new(listening_addr: &str, name: &str) -> Arc<KVSServer> {
        Arc::new(KVSServer {
            listen_addr_: String::from_str(listening_addr).unwrap(),
            kvs_access_: 
                RwLock::new(
                    KeyValueStore::new(name)
                )
            
        })
    }

    pub fn handle_ping_request(&self, binary_req: &[u8]) {
        let ping_request: PingRequest;
        match parse_ping_request(binary_req) {
            Ok(v) => { ping_request = v; },
            Err(e) => {
                eprintln!("Parse error: {:?}", e);
                return;
            }
        };
        let message: String = ping_request.ping_message;
        println!("Received ping: {:?}", message);
    }

    fn add_value(&self, pair: KeyValuePair) -> bool {
        let mut store = self.kvs_access_.write().unwrap();
        let success = (*store).add(kvp_proto_to_kvp_rust(pair));
        if success {
            println!("Successfully added pair!");
        } else {
            println!("Did not add pair!");
        }
        return success;
    }

    pub fn handle_create_request(&self, binary_req: &[u8]) {
        let create_request: CreateKvPairReq;
        match parse_create_request(binary_req) {
            Ok(v) => { create_request = v; },
            Err(e) => {
                eprintln!("Parse error: {:?}", e);
                return;
            }
        }
        if create_request.pair == None {
            return;
        }
        let insertable_pair;
        match create_request.pair {
            None => return,
            Some(x) => { insertable_pair = x; }
        }
        println!("Got key: {:?}", insertable_pair.key.as_str());
        println!("Got value: {:?}", insertable_pair.value.as_str());

        self.add_value(insertable_pair);
    }

    // TODO: Given that Error is a trait, we should ideally create custom
    // errors that extend it and improve our error reporting system.
    pub async fn main_loop(self: Arc<Self>) -> Result<(), Box<dyn std::error::Error>> {
        // Spawn a single listener at the appropriate address
        let listener = TcpListener::bind(
            self.listen_addr_.as_str()).await?;
        // Create an infinite loop that waits on a connection to the socket.
        // Once a connection is hit, spawn off a handler to this connection
        // that reads the data input to the socket, handles it, and exits
        // gracefully.
        loop {
            let self_arc = self.clone();
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
                            let req_type = received_req.req_type();
                            let payload = received_req.payload;
                            match req_type {
                                ReqType::Ping => {
                                    self_arc.handle_ping_request(&payload);
                                },
                                ReqType::Create => {
                                    self_arc.handle_create_request(&payload);
                                },
                                _ => {
                                    println!("Coming soon!")
                                }
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