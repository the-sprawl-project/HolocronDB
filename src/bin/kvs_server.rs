use key_value_store::socket_interface::kvs_server_impl::KVSServer;

use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Hello, server!");
    let server = KVSServer::new("127.0.0.1:8080",
    "default");
    match server.main_loop().await {
        Ok(_) => {}
        Err(e) => { eprintln!("Got error {:?}", e)}
    };
    Ok(())
}
