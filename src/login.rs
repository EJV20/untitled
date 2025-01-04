use serde::{Deserialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn handle_login(body: LoginRequest) -> Result<impl warp::Reply, warp::Rejection> {
    // Logic here to handle the login
    if body.username == "admin" && body.password == "password" {
        Ok(warp::reply::json(&"Login successful"))
    } else {
        Ok(warp::reply::json(&"Invalid credentials"))
    }
}