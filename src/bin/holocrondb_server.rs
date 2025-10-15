use holocron_db::socket_interface::holocron_db_server_impl::HolocronDBServer;

use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Hello, server!");
    let server = HolocronDBServer::new("127.0.0.1:8080",
    "default");
    match server.main_loop().await {
        Ok(_) => {}
        Err(e) => { eprintln!("Got error {:?}", e)}
    };
    Ok(())
}
