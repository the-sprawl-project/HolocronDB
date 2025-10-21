use holocron_db::socket_interface::holocron_db_server_impl::HolocronDBServer;

use std::io;
use log::{trace, info, warn, error};
use std::{env, process::exit};
use serde::Deserialize;
use tokio::fs;
use toml;


#[derive(Deserialize)]
struct Config {
    net_config: NetConfig
}

#[derive(Deserialize)]
struct NetConfig {
    ip: String,
    port: u16
}


async fn parse_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = match fs::read_to_string(path).await {
        Ok(c) => c,
        Err(e) => {return Err(Box::new(e)); }
    };
    let config: Config = toml::from_str(&contents).unwrap();
    Ok(config)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "trace");
    let config_loc = "server_config.toml";
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    let config = match parse_config(config_loc).await {
        Ok(c) => {
            info!("Successfully read config file: {}", config_loc);
            c
        },
        Err(e) => {
            error!("Got error: {:?} trying to read config file {}", e, config_loc);
            exit(1);
        }
    };
    let addr = config.net_config.ip;
    let port = config.net_config.port;
    let listen_addr = format!("{}:{}", addr, port);
    trace!("Hello, server!");
    let server = HolocronDBServer::new(&listen_addr,
    "default");
    match server.main_loop().await {
        Ok(_) => {}
        Err(e) => { warn!("Got error {:?}", e)}
    };
    Ok(())
}
