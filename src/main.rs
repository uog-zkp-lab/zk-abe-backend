// Include necessary crates
use warp::Filter;
use serde::{Deserialize, Serialize};
use rabe::schemes::bsw::*;
use rabe::utils::policy::pest::PolicyLanguage;
use log::{info, LevelFilter};
use simplelog::{Config, TermLogger, TerminalMode, ColorChoice};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use sha2::{Sha256, Digest};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Response {
    message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct PolicySubmission {
    data_owner_id: String,
    attributes: Vec<Attribute>,
    policy: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Attribute {
    name: String,
    value: String,
}

#[derive(Debug, Clone)]
struct MerkleNode {
    hash: String,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    fn new(hash: String) -> Self {
        MerkleNode {
            hash,
            left: None,
            right: None,
        }
    }
}

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
    let mut nodes: Vec<MerkleNode> = attributes.into_iter()
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
        print_tree(left, depth + 1);
    }
    if let Some(ref right) = node.right {
        print_tree(right, depth + 1);
    }
}

#[tokio::main]
async fn main() {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

    let db: Database = Arc::new(Mutex::new(HashMap::new()));

    let hello = warp::path!("hello" / String)
        .map(|name| {
            warp::reply::json(&Response {
                message: format!("Hello, {}!", name),
            })
        });

    let test_policy = warp::path!("test_policy")
        .map(|| {
            info!("test_policy endpoint called");

            // Initialize the BSW CP-ABE scheme
            let (pk, msk) = setup();
            info!("BSW CP-ABE setup completed");

            let plaintext = b"Hello, World!";

            // Define a policy
            let policy = String::from(r#""A" and "B""#);
            let ct_cp = match encrypt(&pk, &policy, PolicyLanguage::HumanPolicy, plaintext) {
                Ok(ct) => {
                    info!("Encryption successful");
                    ct
                },
                Err(e) => {
                    info!("Encryption failed: {:?}", e);
                    return warp::reply::json(&Response {
                        message: "Encryption failed".to_string(),
                    });
                }
            };

            // Generate a key pair with attributes that do not satisfy the policy
            let sk = match keygen(&pk, &msk, &vec!["C", "D"]) {
                Some(sk) => {
                    info!("Key generation successful");
                    sk
                },
                None => {
                    info!("Key generation failed");
                    return warp::reply::json(&Response {
                        message: "Key generation failed".to_string(),
                    });
                }
            };

            let decrypted_plaintext = match decrypt(&sk, &ct_cp) {
                Ok(pt) => {
                    info!("Decryption successful");
                    pt
                },
                Err(e) => {
                    info!("Decryption failed: {:?}", e);
                    return warp::reply::json(&Response {
                        message: "Decryption failed".to_string(),
                    });
                }
            };

            let message = if decrypted_plaintext == plaintext {
                "Policy language test passed!"
            } else {
                "Policy language test failed."
            };

            warp::reply::json(&Response {
                message: message.to_string(),
            })
        });

    let db_filter = warp::any().map(move || db.clone());

    let submit_policy = warp::path("submit_policy")
        .and(warp::post())
        .and(warp::body::json())
        .and(db_filter)
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
        });

    warp::serve(hello.or(test_policy).or(submit_policy))
        .run(([127, 0, 0, 1], 3031))
        .await;
}
