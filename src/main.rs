use warp::Filter;
use serde::{Deserialize, Serialize};
use rabe::schemes::bsw::*;
use rabe::utils::policy::pest::PolicyLanguage;

#[derive(Deserialize, Serialize)]
struct Response {
    message: String,
}

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String)
        .map(|name| {
            warp::reply::json(&Response {
                message: format!("Hello, {}!", name),
            })
        });

    let test_policy = warp::path!("test_policy")
        .map(|| {
            // Initialize the BSW CP-ABE scheme
            let (pk, msk) = setup();
            let plaintext = b"Hello, World!";

            // Define a policy
            let policy = String::from(r#""A" and "B""#);
            let ct_cp = encrypt(&pk, &policy, PolicyLanguage::HumanPolicy, &*plaintext).unwrap(); // Use dereference operator here

            // Generate a key pair
            let sk = keygen(&pk, &msk, &vec!["A", "B"]).unwrap();
            let decrypted_plaintext = decrypt(&sk, &ct_cp).unwrap();

            let message = if decrypted_plaintext == plaintext {
                "Policy language test passed!"
            } else {
                "Policy language test failed."
            };

            warp::reply::json(&Response {
                message: message.to_string(),
            })
        });

    warp::serve(hello.or(test_policy))
        .run(([127, 0, 0, 1], 3031))
        .await;
}
