mod login;
mod signup;

use warp::Filter;
use serde_json::json;
use crate::signup::{create_account, CreateAccountRequest};

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

    let create_account_route = api_base
        .and(warp::path("create_account"))
        .and(warp::post()) // Accept only POST requests
        .and(warp::body::json::<CreateAccountRequest>()) // Parse the body to `CreateAccountRequest`
        .map(|req: CreateAccountRequest| {
            // Call the `create_account` helper method
            let response = create_account(req);

            warp::reply::json(&response)
        });

    // Combine routes and start the server
    let routes = hello_route.or(login_route)
        .or(create_account_route)
        .with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}


