use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    pub authentication_token: String,
}
