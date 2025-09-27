pub mod key_value_store;
pub mod socket_interface;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/key_value_store.rs"));
    include!(concat!(env!("OUT_DIR"), "/socket_messages.rs"));
}