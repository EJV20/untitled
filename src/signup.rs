use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateAccountRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct CreateAccountResponse {
    pub success: bool,
    pub message: String,
}

pub async fn create_account(req: CreateAccountRequest) -> CreateAccountResponse {
    // Get the connection pool
    let pool = match crate::DB_POOL.get() {
        Some(pool) => pool,
        None => {
            return CreateAccountResponse {
                success: false,
                message: "Database pool is not initialized.".to_string(),
            };
        }
    };

    let password_hash = match hash(req.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => {
            return CreateAccountResponse {
                success: false,
                message: "Password hashing failed.".to_string(),
            };
        }
    };

    // Insert into the database
    let result = sqlx::query!(
        r#"
        INSERT INTO users (username, email, password)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        req.username,
        req.email,
        password_hash
    ).fetch_one(pool)
        .await;

    match result {
        Ok(_) => CreateAccountResponse {
            success: true,
            message: "Account created successfully.".into(),
        },
        Err(err) => CreateAccountResponse {
            success: false,
            message: format!("Failed to create account: {}", err),
        },
    }
}

