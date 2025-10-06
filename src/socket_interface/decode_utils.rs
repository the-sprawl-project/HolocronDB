// Importing everything to keep the decode clean.
// TODO(@Skeletrox): Split into req_decoders and resp_decoders?
use crate::proto::*;
use crate::key_value_store::key_value_pair;
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

pub fn parse_create_request(request: &[u8]) -> Result<CreateKvPairReq, SocketError> {
    match CreateKvPairReq::decode(request) {
        Ok(res) => Ok(res),
        Err(e) => Err(SocketError {
            kind_: ErrorKind::ParseError,
            context_: e.to_string()
        })
    }
}

pub fn kvp_proto_to_kvp_rust(inp: KeyValuePair) -> key_value_pair::KeyValuePair {
    key_value_pair::KeyValuePair::new(
        &inp.key,
        &inp.value
    )
}
