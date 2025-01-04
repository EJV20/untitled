mod login;

use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a simple route `/hello`
    let hello = warp::path("hello")
        .map(|| warp::reply::html("Hello, World!"));

    // Start the server on 127.0.0.1:3030
    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}


