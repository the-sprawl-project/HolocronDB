use key_value_store::socket_interface::kvs_client_impl::KVSClient;
use key_value_store::socket_interface::socket_errors::SocketError;

use std::io::{self, Write};


fn print_basic_help() {
    println!("\n=====How to use this=====");
    println!("c <key> <value>: Creates simple key value pair");
    println!("r <key>: Reads a key from the key value store");
    println!("p <message>: Pings the key value store with a message");
    println!("u <key> <value>: Updates the key value store with new value");
    println!("x: Exits the client");
    println!("=========================\n");

}

#[tokio::main]
async fn main() -> Result<(), SocketError> {
    let prompt_prefix = ">> ";
    let mut input = String::new();
    let mut exit_loop = false;
    let mut client = KVSClient::new("127.0.0.1:8080").await?;
    println!(
        "KV Store client!!\n--------\nSend x to exit, h for help\n-------\n");
    while !exit_loop {
        print!("{}", prompt_prefix);
        let _ = io::stdout().flush();
        input.clear();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let ip = input.trim();
        let control_char = ip.chars().nth(0).unwrap();
        let mut skip_input = false;
        match control_char {
            'x' => {
                exit_loop = true;
                skip_input = true;
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
            'r' => {
                let mut split = ip.split(' ');
                split.next();
                let read_key: &str;
                match split.next() {
                    None => {
                        eprintln!("Expected key to read!");
                        break;
                    }
                    Some(x) => {read_key = x; }
                }
                client.send_read(read_key).await?;
            },
            'u' => {
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
                client.send_update(key, val).await?;
            },
            'h' => {
                print_basic_help();
                skip_input = true;
            },
            _ => {
                eprintln!("Unexpected input: {:?}", ip);
                skip_input = true;
            }
        }
        if !skip_input {
            match client.receive_resp().await {
                Ok(s) => println!("<< {}", s),
                Err(e) => eprintln!("<! {}", e)
            }
        }
    }
        
    Ok(())
}
