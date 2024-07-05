use crate::models::{MerkleNode, PolicySubmission, Response};
use log::info;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp::Filter;

type Database = Arc<Mutex<HashMap<String, PolicySubmission>>>;

fn parse_policy(policy: &str) -> Vec<&str> {
    policy.split_whitespace().collect()
}

fn hash_data(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

fn build_merkle_tree(attributes: Vec<&str>) -> MerkleNode {
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

fn print_tree(node: &MerkleNode, depth: usize) {
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

pub fn submit_policy_filter(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("submit_policy")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || db.clone()))
        .map(|policy_submission: PolicySubmission, db: Database| {
            info!("Policy submission received: {:?}", policy_submission);

            // Validate the policy
            if policy_submission.policy.is_empty() || policy_submission.attributes.is_empty() {
                return warp::reply::json(&Response {
                    message: "Invalid policy submission".to_string(),
                });
            }

            // Parse and build Merkle Tree from the policy
            let attributes = parse_policy(&policy_submission.policy);
            let merkle_root = build_merkle_tree(attributes);
            info!("Merkle Root: {:?}", merkle_root);
            println!("Merkle Tree:");
            print_tree(&merkle_root, 0);

            // Store the policy
            let mut db = db.lock().unwrap();
            db.insert(policy_submission.data_owner_id.clone(), policy_submission);

            warp::reply::json(&Response {
                message: "Policy submitted successfully".to_string(),
            })
        })
}
