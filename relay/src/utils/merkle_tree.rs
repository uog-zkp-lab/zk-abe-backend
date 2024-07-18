use crate::types::models::MerkleNode;
use risc0_zkvm::sha::{Digest, Impl, Sha256};

pub fn parse_policy(policy: &str) -> Vec<&str> {
    policy.split_whitespace().collect()
}

pub fn hash_data(data: &str) -> String {
    let h = *Impl::hash_bytes(&data.as_bytes());
    h.to_string()
}

pub fn build_merkle_tree(attributes: Vec<&str>) -> MerkleNode {
    let mut nodes: Vec<MerkleNode> = attributes
        .into_iter()
        .map(|attr| MerkleNode::new(hash_data(attr)))
        .collect();

    while nodes.len() > 1 {
        let mut new_level = vec![];
        for i in (0..nodes.len()).step_by(2) {
            if i + 1 < nodes.len() {
                let combined_hash = hash_data(&(nodes[i].hash.clone() + &nodes[i + 1].hash));
                let mut parent = MerkleNode::new(combined_hash);
                parent.left = Some(Box::new(nodes[i].clone()));
                parent.right = Some(Box::new(nodes[i + 1].clone()));
                new_level.push(parent);
            } else {
                new_level.push(nodes[i].clone());
            }
        }
        nodes = new_level;
    }

    nodes[0].clone()
}

pub fn print_tree(node: &MerkleNode, depth: usize) {
    for _ in 0..depth {
        print!("  ");
    }
    println!("Hash: {}", node.hash);
    if let Some(ref left) = node.left {
        for _ in 0..(depth + 1) {
            print!("  ");
        }
        println!("Left Node:");
        print_tree(left, depth + 2);
    }
    if let Some(ref right) = node.right {
        for _ in 0..(depth + 1) {
            print!("  ");
        }
        println!("Right Node:");
        print_tree(right, depth + 2);
    }
}
