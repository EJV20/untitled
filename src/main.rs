mod login;

use warp::Filter;

const HELLO_RESPONSE: &str = "Hello, World!";

#[tokio::main]
async fn main() {
    let api_base = warp::path("api");

    let hello_route = api_base.clone()
        .and(warp::path("hello"))
        .map(|| warp::reply::html(HELLO_RESPONSE));

    let login_route = api_base
        .and(warp::path("login"))
        .and(warp::post()) // Accept only POST requests
        .and(warp::body::json()) // Parse the body as JSON
        .and_then(login::handle_login);

    // Combine routes and start the server
    let routes = hello_route.or(login_route);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}


