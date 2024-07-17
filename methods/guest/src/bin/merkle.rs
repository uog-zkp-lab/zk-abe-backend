use json::parse;
use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256}; // for sha256 hashing
use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    pub data: u32,
    pub hash: Digest,
}

risc0_zkvm::guest::entry!(main);

fn main() {
    // input keys (ACP tree + attributes + MTR)
    // calculate the merkle root of the ACP tree through the attributes
    // assert_eq!(merkle_root, MTR);
    // output with something

    let data: String = env::read();
    let sha = *Impl::hash_bytes(&data.as_bytes());
    let data = parse(&data).unwrap();
    let proven_val = data["uid"].as_u32().unwrap();
    assert_eq!(sha.to_string(), "a32e2eeec7f0eb058610965ae38adc5a7bc5ccfa66f27ea400062f6c6a5b0ee7");
    let out = Outputs {
        data: proven_val,
        hash: sha,
    };
    env::commit(&out);
}
