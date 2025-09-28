// Importing everything to keep the decode clean.
// TODO(@Skeletrox): Split into req_decoders and resp_decoders?
use crate::proto::*;
use prost::Message;
use super::socket_errors::{SocketError, ErrorKind};

pub fn parse_generic_request(request: &[u8]) -> Result<GenericRequest, SocketError> {
    match GenericRequest::decode(request) {
        Ok(res) => Ok(res),
        Err(e) => Err(SocketError {
            kind_: ErrorKind::ParseError,
            context_: e.to_string()
        })
    }
}

pub fn parse_ping_request(request: &[u8]) -> Result<PingRequest, SocketError> {
    match PingRequest::decode(request) {
        Ok(res) => Ok(res),
        Err(e) => Err(SocketError {
            kind_: ErrorKind::ParseError,
            context_: e.to_string()
        })
    }
}
