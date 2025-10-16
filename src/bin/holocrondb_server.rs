use holocron_db::socket_interface::holocron_db_server_impl::HolocronDBServer;

use std::io;
use log::{trace, warn};

#[tokio::main]
async fn main() -> io::Result<()> {
    trace!("Hello, server!");
    let server = HolocronDBServer::new("127.0.0.1:8080",
    "default");
    match server.main_loop().await {
        Ok(_) => {}
        Err(e) => { warn!("Got error {:?}", e)}
    };
    Ok(())
}
