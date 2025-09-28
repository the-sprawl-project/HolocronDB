use key_value_store::socket_interface::kvs_client_impl::KVSClient;

use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let client = KVSClient::new("127.0.0.1:8080");
    client.send_ping("Hello there").await?;
    Ok(())
}
