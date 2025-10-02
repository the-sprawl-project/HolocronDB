use std::str::FromStr;

use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use prost::Message;
use crate::proto::{GenericRequest, PingRequest, ReqType};


// The client does not support persistent connections as of yet.
// For that, we will need to persist the stream between connections
// and make the senders accept it as an input.
pub struct KVSClient {
    _server_addr: String,
}

impl KVSClient {
    pub async fn new(server_addr: &str) -> std::io::Result<KVSClient> {
        Ok(KVSClient {
            _server_addr: String::from_str(server_addr).expect(
                "Could not parse string!"),
        })
    }

    pub async fn send_ping(&self, message: &str) -> std::io::Result<bool> {
        let mut stream = TcpStream::connect(self._server_addr.clone()).await?;
        let mut request = GenericRequest::default();
        let mut ping_request = PingRequest::default();
        ping_request.ping_message = String::from_str(message).expect(
            "Cannot parse string");
        request.set_req_type(ReqType::Ping);
        let binaried_ping = ping_request.encode_to_vec();
        request.payload = binaried_ping;
        let binaried_req = request.encode_to_vec();
        stream.write_all(&binaried_req[..]).await?;
        Ok(true)
    }
}