// src/test_policy.rs
use crate::models::Response;
use log::info;
use rabe::schemes::bsw::*;
use rabe::utils::policy::pest::PolicyLanguage;
use warp::Filter;

pub fn test_policy_filter(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("test_policy").map(|| {
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
            }
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
            }
            None => {
                info!("Key generation failed");
                return warp::reply::json(&Response {
                    message: "Key generation failed".to_string(),
                });
            }
        };

        // Attempt to decrypt the ciphertext with the generated key
        let decrypted_plaintext = match decrypt(&sk, &ct_cp) {
            Ok(pt) => {
                info!("Decryption successful");
                pt
            }
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
    })
}
