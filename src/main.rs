mod login;

use warp::Filter;
use serde_json::json;

const HELLO_RESPONSE: &str = "Hello, World!";

#[tokio::main]
async fn main() {
    let api_base = warp::path("api");

    // Create CORS filter
    let cors = warp::cors()
        .allow_any_origin() // Allow requests from any origin
        .allow_method("GET") // Allow GET requests
        .allow_method("POST") // Allow POST requests
        .allow_header("Content-Type"); // Allow the `Content-Type` header

    let hello_route = api_base.clone()
        .and(warp::path("hello"))
        .map(|| warp::reply::json(&json!({ "message": HELLO_RESPONSE })));

    let login_route = api_base
        .and(warp::path("login"))
        .and(warp::post()) // Accept only POST requests
        .and(warp::body::json()) // Parse the body as JSON
        .and_then(login::handle_login);

    // Combine routes and start the server
    let routes = hello_route.or(login_route).with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}


