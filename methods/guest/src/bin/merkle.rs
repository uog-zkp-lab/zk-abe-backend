use std::io::Read;

// use alloy_primitives::U256;
// use alloy_sol_types::SolValue;
use risc0_zkvm::guest::env;
use json::parse;

fn main() {
    // another security design:
    // since we want to check the validity of data processor, it is possible to add another signature check for its registered public key pair.

    // inputs: 
    // 1. ACP provided by Data Processor
    // 2. uid of the user
    //    * ACP Tree
    //    * ACP Merkle Root

    // 0. the inputs will be reading just the attributes that we want
    //    , which means that DP won't have to worry about to operate 
    //      the data.

    // Step 1: Reading the whole json file from dp's local file system
    let uid: String = env::read(); // unique identifier for data owner
    let dp_id_tree: String = env::read(); // the whole identity tree json string 

    println!("UID: {:?}", uid);
    println!("dp id tree: {:?}", dp_id_tree);

    // TODO: Step 2: Query ACP Keys from db
    
    // TODO: Step 3: Building the merkle tree from ACP
    
    // TODO: Step 4: Query the MTR from db
    
    // TODO: Step 5: Assert the MTR with the calculated MTR
    
    // TODO: Step 6: Output and commit with journal (ABI_ENCODED) and MTR
}
