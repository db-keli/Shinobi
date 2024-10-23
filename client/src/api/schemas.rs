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

#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    pub authentication_token: String,
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
