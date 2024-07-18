use crate::types::models::{PolicySubmission, Response};
use log::info;
use methods::{MERKLE_ELF, MERKLE_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use warp::Filter;

pub fn test_zkvm_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path("query_for_uid")
        .and(warp::post())
        .and(warp::body::json())
        // .and(warp::any().map(move || db.clone()))
        .map(|policy_submission: PolicySubmission| {
            info!("Test ZKVM endpoint called");
            println!("Policy Submission: {:?}", policy_submission);
            // env_logger::init();
            // Parse CLI Arguments: The application starts by parsing command-line arguments provided by the user.
            // let args = Args::parse();

            // // Create a new transaction sender using the parsed arguments.
            // let tx_sender = TxSender::new(
            //     args.chain_id,
            //     &args.rpc_url,
            //     &args.eth_wallet_private_key,
            //     &args.contract,
            // )?;

            // ABI encode input: Before sending the proof request to the Bonsai proving service,
            // the input number is ABI-encoded to match the format expected by the guest code running in the zkVM.
            // let input = args.input.abi_encode();
            let uid = "76178887";
            let data = include_str!("../../../template_data/dp_1_data.json");

            let env = ExecutorEnv::builder()
                .write(&uid)
                .unwrap()
                .write(&data)
                .unwrap()
                .build()
                .expect("Failed to build ExecutorEnv");

            let prover = default_prover();
            let receipt = prover.prove(env, MERKLE_ELF).unwrap().receipt;

            println!("reciept: {:?}", receipt);

            let message = "Nice work!";

            warp::reply::json(&Response {
                message: message.to_string(),
            })
        })
}
