mod models;
mod submit_policy;
mod test_policy;

use crate::models::{PolicySubmission, Response};
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp::Filter;

type Database = Arc<Mutex<HashMap<String, PolicySubmission>>>;

#[tokio::main]
async fn main() {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

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
