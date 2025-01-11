mod login;
mod signup;

use warp::Filter;
use serde_json::json;
use crate::signup::{create_account, CreateAccountRequest};
use sqlx::PgPool;
use tokio::sync::OnceCell;

const HELLO_RESPONSE: &str = "Hello, World!";

static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create PostgreSQL connection pool
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // Set the global connection pool
    DB_POOL.set(pool).expect("Failed to set the database pool");

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
        .and_then(|req: CreateAccountRequest| async move {
            // Call the `create_account` helper method asynchronously
            let response = create_account(req).await;

            // Reply with JSON
            Ok::<_, warp::Rejection>(warp::reply::json(&response))
        });

    // Combine routes and start the server
    let routes = hello_route.or(login_route)
        .or(create_account_route)
        .with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}


