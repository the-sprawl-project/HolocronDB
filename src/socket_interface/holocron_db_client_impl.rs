use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use prost::Message;
use crate::proto::*;
use super::decode_utils::parse_generic_response;
use super::socket_errors::{SocketError, ErrorKind};
use log::warn;

pub struct HolocronDBClient {
    _server_addr: String,
    _framed: Framed<TcpStream, LengthDelimitedCodec>
}

impl HolocronDBClient {
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

    pub async fn send_ping(&mut self, message: &str) -> Result<bool, SocketError> {
        let mut request = GenericRequest::default();
        let mut ping_request = PingRequest::default();
        ping_request.ping_message = message.to_string();
        request.set_req_type(ReqType::Ping);
        request.payload = ping_request.encode_to_vec();
        self.send_message(request).await?;
        Ok(true)
    }

    pub async fn send_create(&mut self, key: &str, val: &str) -> Result<bool, SocketError> {
        let mut request = GenericRequest::default();
        let mut create_req = CreateKvPairReq::default();
        let mut pair = KeyValuePair::default();
        pair.key = String::from(key);
        pair.value = String::from(val);
        create_req.pair = Some(pair);
        request.payload = create_req.encode_to_vec();
        request.set_req_type(ReqType::Create);
        self.send_message(request).await?;
        Ok(true)
    }

    pub async fn send_delete(&mut self, key: &str) -> Result<bool, SocketError> {
        let mut request = GenericRequest::default();
        let mut delete_req = DeleteKvPairReq::default();
        delete_req.key = String::from(key);
        request.payload = delete_req.encode_to_vec();
        request.set_req_type(ReqType::Delete);
        self.send_message(request).await?;
        Ok(true)
    }

    pub async fn send_update(&mut self, key: &str, val: &str) -> Result<bool, SocketError> {
        let mut request = GenericRequest::default();
        let mut update_req = UpdateKvPairReq::default();
        let mut pair = KeyValuePair::default();
        pair.key = String::from(key);
        pair.value = String::from(val);
        update_req.pair = Some(pair);
        request.payload = update_req.encode_to_vec();
        request.set_req_type(ReqType::Update);
        self.send_message(request).await?;
        Ok(true)
    }

    pub async fn send_read(&mut self, key: &str) -> Result<bool, SocketError> {
        let mut request = GenericRequest::default();
        let mut read_req = ReadKvPairReq::default();
        read_req.key = key.to_string();
        request.payload = read_req.encode_to_vec();
        request.set_req_type(ReqType::Read);
        self.send_message(request).await?;
        Ok(true)
    }

    pub async fn send_backup(&mut self, backup_id: &str) -> Result<bool, SocketError> {
        let mut request = GenericRequest::default();
        let mut read_req = BackupReq::default();
        read_req.backup_id = backup_id.to_string();
        request.payload = read_req.encode_to_vec();
        request.set_req_type(ReqType::Backup);
        self.send_message(request).await?;
        Ok(true)
    }

    pub async fn receive_resp(&mut self) -> Result<String, SocketError> {
        if let Some(Ok(bytes)) = self._framed.next().await {
            match parse_generic_response(&bytes.freeze()) {
                Ok(x) => return Ok(x),
                Err(e) => return Err(e)
            }
        } else {
            warn!("Connection closed!");
        }
        Ok("".to_string())
    }
}