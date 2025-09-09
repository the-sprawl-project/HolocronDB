extern crate prost_build;
use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &["src/proto/key_value_store.proto"],
        &["src/"]).unwrap();
    Ok(())
}
