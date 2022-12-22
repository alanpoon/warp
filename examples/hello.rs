#![deny(warnings)]
use warp::Filter;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
