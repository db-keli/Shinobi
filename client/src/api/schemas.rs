use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthResponse {
    pub authentication_token: TokenData,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct TokenData {
    pub token: String,
    pub expiry: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectInput {
    pub name: String,
    #[serde(rename = "project_url")]
    pub project_url: String,
    #[serde(rename = "build_commands")]
    pub build_commands: Vec<String>,
    pub keys: HashMap<String, String>,
    #[serde(rename = "expire_at")]
    pub expire_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct AllowUserInput {
    pub project_name: String,
    pub user_email: String,
}

#[derive(Serialize, Clone)]
pub struct GetKeysInput {
    #[serde(rename = "project_name")]
    pub project_name: String,
    #[serde(rename = "token")]
    pub token: String,
}
