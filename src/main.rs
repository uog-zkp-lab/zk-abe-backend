use warp::Filter;
use serde::{Deserialize, Serialize};

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

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;

        println!("Hello, world!");
}

