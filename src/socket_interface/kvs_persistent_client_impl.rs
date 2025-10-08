use std::pin::Pin;
use std::str::FromStr;

use futures::{SinkExt, StreamExt};
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use prost::Message;
use crate::proto::*;
use super::socket_errors::{SocketError, ErrorKind};

pub struct KVSPersistentClient {
    _server_addr: String,
    _framed: Framed<TcpStream, LengthDelimitedCodec>
}

impl KVSPersistentClient {
    pub async fn new(addr: &str) -> Result<Self, SocketError> {
        let stream;
        match TcpStream::connect(addr).await {
            Ok(x) => stream = x,
            Err(e) => return Err(SocketError {
                kind_: ErrorKind::ConnectError,
                context_: e.to_string()
            })
        }
        let framed = Framed::new(stream, LengthDelimitedCodec::new());
        Ok(Self { _server_addr: String::from(addr),
                  _framed: framed })
    }

    pub async fn send_message(
            &mut self, req: GenericRequest) -> Result<(), SocketError> {
        let bytes = req.encode_to_vec();
        match self._framed.send(bytes.into()).await {
            Ok(_) => return Ok(()),
            Err(e) => return Err(SocketError { kind_: ErrorKind::ConnectError,
                context_: e.to_string() })
        };
    }

    pub async fn send_ping(&mut self, message: &str) -> std::io::Result<bool> {
        let mut request = GenericRequest::default();
        let mut ping_request = PingRequest::default();
        ping_request.ping_message = message.to_string();
        request.set_req_type(ReqType::Ping);
        request.payload = ping_request.encode_to_vec();
        self.send_message(request);
        Ok(true)
    }

    pub async fn send_create(&mut self, key: &str, val: &str) -> std::io::Result<bool> {
        let mut request = GenericRequest::default();
        let mut create_req = CreateKvPairReq::default();
        let mut pair = KeyValuePair::default();
        pair.key = String::from(key);
        pair.value = String::from(val);
        create_req.pair = Some(pair);
        request.payload = create_req.encode_to_vec();
        request.set_req_type(ReqType::Create);
        self.send_message(request);
        Ok(true)
    }

    pub async fn receive_resp(&mut self) -> std::io::Result<()> {
        if let Some(Ok(bytes)) = self._framed.next().await {
            println!("Got response");
        } else {
            eprintln!("Connection closed!");
        }
        Ok(())
    }
}