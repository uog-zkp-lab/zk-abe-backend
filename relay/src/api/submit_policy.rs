use crate::{
    types::models::{Database, PolicySubmission, Response},
    utils::merkle_tree::{build_merkle_tree, parse_policy, print_tree},
};
use log::info;
use warp::Filter;

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
