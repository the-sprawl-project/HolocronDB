use super::key_value_store::KeyValueStore;
use super::key_value_pair::KeyValuePair;
use super::errors;

use std::fs::File;
use std::io::prelude::*;

use prost::Message;
use crate::{key_value_store::errors::{ErrorKind, RWError},
    proto::KeyValueStoreMsg};


fn key_value_store_to_msg(k: KeyValueStore) -> KeyValueStoreMsg {
    let mut msg = KeyValueStoreMsg::default();
    msg.name = k.name().to_string();
    for v in k.all() {
        msg.values.insert(v.0, v.1);
    }
    return msg;
}

fn msg_to_key_value_store(m: KeyValueStoreMsg) -> KeyValueStore {
    let mut store =  KeyValueStore::new(&m.name);
    for pair in m.values {
        store.add(KeyValuePair::new(
            &pair.0, &pair.1));
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


#[cfg(test)]
mod tests {
    use super::*;

    fn create_simple_kv_store() -> KeyValueStore {
        let mut kvs = KeyValueStore::new("test");
        kvs.add(KeyValuePair::new("Hello", "Test"));
        kvs.add(KeyValuePair::new("Goodbye", "Test"));
        return kvs;
    }

    #[test]
    fn test_obj_to_msg_to_obj() {
        let kvs = create_simple_kv_store();
        let msg = key_value_store_to_msg(kvs.clone());
        let kvs_2 = msg_to_key_value_store(msg);

        assert_eq!(kvs.name(), kvs_2.name());

        for (k, v) in kvs.all() {
            if let Some(val) = kvs_2.get(&k.clone()) {
                assert_eq!(v, val.value());
            } else {
                assert!(false, "No value found for key {:?}!", k);
            }
        }
    }
}