use crate::api::schemas::{AuthRequest, AuthResponse, CreateUserInput, ProjectInput};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde_json::Value;
use std::error::Error;

pub struct ApiService {
    client: Client,
    base_url: String,
    token: String,
}

impl ApiService {
    pub fn new(base_url: &str, token: &str) -> Self {
        ApiService {
            client: Client::new(),
            base_url: base_url.to_string(),
            token: token.to_string(),
        }
    }

    pub async fn get_health(&self) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/health", self.base_url);

        // Create a header map and set the Authorization header
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token))?,
        );

        // Send the GET request with headers
        let response = self.client.get(&url).headers(headers).send().await?;

        // Ensure the request was successful
        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            Err(format!("Failed to get health: {}", response.status()).into())
        }
    }

    pub async fn create_account(
        &self,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/users/register", self.base_url);

        let payload = CreateUserInput {
            name: name.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        };

        let response = self.client.post(url).json(&payload).send().await?;

        match response.status() {
            reqwest::StatusCode::CREATED => {
                println!("Account created successfully!");
                Ok(())
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let err_msg: serde_json::Value = response.json().await?;
                println!("Bad request: {:?}", err_msg);
                Err("Failed to create account: bad request".into())
            }
            reqwest::StatusCode::CONFLICT => {
                let err_msg: serde_json::Value = response.json().await?;
                println!("Conflict: {:?}", err_msg);
                Err("Failed to create account: user already exists".into())
            }
            _ => {
                let err_msg: serde_json::Value = response.json().await?;
                println!("Error: {:?}", err_msg);
                Err("Failed to create account: unknown error".into())
            }
        }
    }

    pub async fn get_authentication_token(
        &self,
        email: &str,
        password: &str,
    ) -> Result<AuthResponse, Box<dyn Error>> {
        let url = format!("{}/auth/token", self.base_url);

        let payload = AuthRequest {
            email: email.to_string(),
            password: password.to_string(),
        };

        let response = self.client.post(&url).json(&payload).send().await?;

        if response.status().is_success() {
            let auth_response: AuthResponse = response.json().await?;
            println!("Authentication token: {:?}", auth_response);
            Ok(auth_response)
        } else if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            println!("Invalid credentials provided.");
            Err("Invalid credentials".into())
        } else {
            let error_msg: serde_json::Value = response.json().await?;
            println!("Failed to authenticate: {:?}", error_msg);
            Err("Authentication failed".into())
        }
    }

    pub async fn create_project(
        &self,
        project_input: ProjectInput, // Use ProjectInput struct here
    ) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/projects/create", self.base_url);

        // Create headers
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token))?,
        );

        // Send the POST request with the serialized ProjectInput struct
        let response = self
            .client
            .post(&url)
            .headers(headers)
            .json(&project_input) // Serialize the struct into JSON
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::CREATED {
            let project: Value = response.json().await?;
            Ok(project)
        } else {
            let error_msg: Value = response.json().await?;
            Err(format!("Failed to create project: {:?}", error_msg).into())
        }
    }

    // Additional methods for other endpoints can be added here
}
