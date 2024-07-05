use host::Outputs;
use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256}; // for sha256 hashing

fn main() {
    // input keys (ACP tree + attributes + MTR)
    // calculate the merkle root of the ACP tree through the attributes
    // assert_eq!(merkle_root, MTR);
    // output with something
    let data: String = env::read();
    let sha = *Impl::hash_bytes(&data.as_bytes());
    let data = parse(&data).unwrap();
    let proven_value = data["critical_data"].as_u32().unwrap();
    let out = Outputs {
        data: proven_value,
        hash: sha,
    };
    env::commit(out);
}
