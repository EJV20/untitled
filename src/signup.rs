use serde::{Deserialize, Serialize};

const HELLO_RESPONSE: &str = "Hello, World!";

// Define the structure of the incoming request for account creation
#[derive(Deserialize)]
pub struct CreateAccountRequest {
    username: String,
    password: String,
}

// Define the structure of the API response
#[derive(Serialize)]
pub struct ApiResponse {
    success: bool,
    message: String,
}

// Helper method to create a new account
pub fn create_account(req: CreateAccountRequest) -> ApiResponse {
    if req.username.is_empty() || req.password.is_empty() {
        // If invalid data, respond with an error
        ApiResponse {
            success: false,
            message: "Username and password are required.".to_string(),
        }
    } else {
        // Normally, you'd store this information in a database here
        // Simulating account creation
        println!(
            "Account created: username = {}, password = (hidden)",
            req.username
        );

        // Respond with success
        ApiResponse {
            success: true,
            message: format!("Account created for username: {}", req.username),
        }
    }
}