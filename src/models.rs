// src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Response {
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PolicySubmission {
    pub data_owner_id: String,
    pub attributes: Vec<Attribute>,
    pub policy: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct MerkleNode {
    pub hash: String,
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    pub fn new(hash: String) -> Self {
        MerkleNode {
            hash,
            left: None,
            right: None,
        }
    }
}
