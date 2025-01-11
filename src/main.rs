mod login;
mod signup;

use crate::signup::{create_account, CreateAccountRequest};
use serde_json::json;
use sqlx::PgPool;
use tokio::sync::OnceCell;
use warp::Filter;

const HELLO_RESPONSE: &str = "Hello, World!";

static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

/// The entry point of the application.
///
/// This function sets up the environment, initializes the database connection pool,
/// defines the API routes, and starts the Warp web server.
#[tokio::main]
async fn main() {
    // Load environment variables from .env file, if present.
    dotenv::dotenv().ok();

    // Retrieve the database connection URL from environment variables.
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a PostgreSQL connection pool.
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // Set the global database connection pool.
    DB_POOL.set(pool).expect("Failed to set the database pool");

    // Define the base API path.
    let api_base = warp::path("api");

    // Create a CORS filter to allow requests from any origin with the specified methods and headers.
    let cors = warp::cors()
        .allow_any_origin() // Allow requests from any origin.
        .allow_method("GET") // Allow HTTP GET requests.
        .allow_method("POST") // Allow HTTP POST requests.
        .allow_header("Content-Type"); // Allow the `Content-Type` header.

    // Define the /api/hello route which returns a simple JSON response.
    let hello_route = api_base.clone()
        .and(warp::path("hello"))
        .map(|| warp::reply::json(&json!({ "message": HELLO_RESPONSE })));

    // Define the /api/login route for user login handling as a POST request.
    let login_route = api_base
        .and(warp::path("login"))
        .and(warp::post()) // Accept only HTTP POST requests.
        .and(warp::body::json()) // Parse the request body as JSON.
        .and_then(login::handle_login);

    // Define the /api/create_account route for creating user accounts as a POST request.
    let create_account_route = api_base
        .and(warp::path("create_account"))
        .and(warp::post()) // Accept only HTTP POST requests.
        .and(warp::body::json::<CreateAccountRequest>()) // Parse the request body into `CreateAccountRequest`.
        .and_then(|req: CreateAccountRequest| async move {
            // Call the `create_account` function asynchronously to process the request.
            let response = create_account(req).await;

            // Reply with the response as JSON.
            Ok::<_, warp::Rejection>(warp::reply::json(&response))
        });

    // Combine all routes and apply the CORS filter.
    let routes = hello_route.or(login_route)
        .or(create_account_route)
        .with(cors);

    // Start the Warp server on localhost at port 3030, serving the defined routes.
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}


