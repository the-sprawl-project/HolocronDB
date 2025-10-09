use key_value_store::socket_interface::kvs_client_impl::KVSClient;
use key_value_store::socket_interface::socket_errors::SocketError;

use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), SocketError> {
    let prompt_prefix = ">> ";
    let mut input = String::new();
    let mut exit_loop = false;
    let mut client = KVSClient::new("127.0.0.1:8080").await?;
    println!("KV Store client!!\n--------\nSend x to exit\n-------\n");
    while !exit_loop {
        print!("{}", prompt_prefix);
        let _ = io::stdout().flush();
        input.clear();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let ip = input.trim();
        let control_char = ip.chars().nth(0).unwrap();

        match control_char {
            'x' => {
                exit_loop = true
            },
            'c' => {
                let mut split = ip.split(' ');
                split.next();
                let key: &str;
                let val: &str;
                match split.next() {
                    None => { 
                        eprintln!("Expected key!");
                        break;
                    },
                    Some(x) => { key = x; }
                }
                match split.next() {
                    None => {
                        eprintln!("Expected value!");
                        break;
                    },
                    Some(x) => { val = x; }
                }
                client.send_create(key, val).await?;
            },
            'p' => {
                let mut split = ip.split(' ');
                split.next();
                let ping_msg: &str;
                match split.next() {
                    None => {
                        eprintln!("Expected message to ping!");
                        break;
                    },
                    Some(x) => { ping_msg = x; }
                }
                client.send_ping(ping_msg).await?;
            },
            _ => {
                eprintln!("Unexpected input: {:?}", ip);
            }
        }
    }
        
    Ok(())
}
