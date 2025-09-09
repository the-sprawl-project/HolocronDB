mod key_value_store;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/key_value_store.rs"));
}

fn main() {
    println!("Hello, world!");
}
