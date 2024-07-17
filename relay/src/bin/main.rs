use log::LevelFilter;
use relay::{
    policy::{submit_policy, test_policy},
    types::models::{PolicySubmission, Response},
};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp::Filter;

type Database = Arc<Mutex<HashMap<String, PolicySubmission>>>;

#[tokio::main]
async fn main() {
    match TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ) {
        Ok(_) => println!("Logger initialized successfully."),
        Err(e) => println!("Logger initialization failed: {:?}", e),
    }

    let db: Database = Arc::new(Mutex::new(HashMap::new()));

    let hello = warp::path!("hello" / String).map(|name| {
        warp::reply::json(&Response {
            message: format!("Hello, {}!", name),
        })
    });

    let routes = hello
        .or(test_policy::test_policy_filter())
        .or(submit_policy::submit_policy_filter(db));

    warp::serve(routes).run(([127, 0, 0, 1], 3031)).await;
}
