mod models;
mod test_policy;
mod submit_policy;

use warp::Filter;
use log::LevelFilter; 
use simplelog::{Config, TermLogger, TerminalMode, ColorChoice}; 
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::models::{PolicySubmission, Response};

type Database = Arc<Mutex<HashMap<String, PolicySubmission>>>;

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

    let routes = hello
        .or(test_policy::test_policy_filter())
        .or(submit_policy::submit_policy_filter(db));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3031))
        .await;
}
