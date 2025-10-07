use key_value_store::socket_interface::kvs_client_impl::KVSClient;

use std::io::{self, Write};

#[tokio::main]
async fn main() -> io::Result<()> {
    let prompt_prefix = ">> ";
    let mut input = String::new();
    let mut exit_loop = false;
    let client = KVSClient::new("127.0.0.1:8080").await?;
    println!("KV Store Pinger!!\n--------\nSend x to exit\n-------\n");
    while !exit_loop {
        print!("{}", prompt_prefix);
        io::stdout().flush();
        input.clear();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let ping_msg = input.trim();
        if ping_msg == "x" {
            exit_loop = true;
        } else if ping_msg == "c" {
            client.send_create("hello", "there").await?;
        } else {
            client.send_ping(ping_msg).await?;
        }
    }
        
    Ok(())
}
