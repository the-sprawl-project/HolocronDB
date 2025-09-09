use super::key_value_store::KeyValueStore;
use super::key_value_pair::KeyValuePair;
use super::errors;

use std::fs::File;
use std::io::prelude::*;

use prost::Message;
use crate::{key_value_store::errors::{ErrorKind, RWError},
    proto::{KeyValuePairMsg, KeyValueStoreMsg}};


fn key_value_store_to_msg(k: KeyValueStore) -> KeyValueStoreMsg {
    let mut msg = KeyValueStoreMsg::default();
    msg.name = k.name().to_string();
    for v in k.all() {
        let mut pair = KeyValuePairMsg::default();
        pair.key = v.0;
        pair.value = v.1;
        msg.values.push(pair);
    }
    return msg;
}

fn msg_to_key_value_store(m: KeyValueStoreMsg) -> KeyValueStore {
    let mut store =  KeyValueStore::new(&m.name);
    for pair in m.values {
        store.add(KeyValuePair::new(
            &pair.key, &pair.value));
    }
    return store;
}

pub fn write_to_file(store: KeyValueStore, target_file: &str) -> Result<(), errors::RWError> {
    let msg = key_value_store_to_msg(store);
    let mut bytes = vec![];
    msg.encode_length_delimited(&mut bytes).unwrap();
    let mut file;
    match File::create(target_file) {
        Ok(f) => { file = f; }
        Err(e) => { return Err(RWError{
            kind_: ErrorKind::FileReadError,
            context_: e.to_string()
        })}
    };
    match file.write_all(&bytes) {
        Ok(_) => { }
        Err(e) => { return Err(RWError
            { kind_: ErrorKind::FileWriteError, context_: e.to_string() })}
    };
    Ok(())
}

pub fn read_from_file(src_file: &str) -> Result<KeyValueStore, errors::RWError> {
    let mut file;
    match File::open(src_file) {
        Ok(f) => { file = f; }
        Err(e) => { return Err(RWError{
            kind_: ErrorKind::FileOpenError, context_: e.to_string()
        })}
    };
    let mut buf =  vec![];
    match file.read(&mut buf) {
        Ok(_) => {},
        Err(e) => {return Err(RWError {
            kind_: ErrorKind::FileReadError,
            context_: e.to_string() })}
    };
    match KeyValueStoreMsg::decode_length_delimited(&buf[..]) {
        Ok(msg) => Ok(msg_to_key_value_store(msg)),
        Err(e) => Err(RWError { 
            kind_: ErrorKind::DataDecodeError,
            context_: e.to_string() })
    }
}