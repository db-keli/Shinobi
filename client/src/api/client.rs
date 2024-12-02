use crate::api::schemas::{
    AllowUserInput, AuthRequest, AuthResponse, CreateUserInput, GetKeysInput, ProjectInput,
};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde_json::Value;
use shinobi_secrets_server::server::server::{self, SecretsServer};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::copy;

use serde_json::json;
pub struct ApiService {
    client: Client,
    base_url: String,
    token: Option<String>,
}
use log::error;

impl ApiService {
    pub fn new(base_url: &str, token: Option<String>) -> Self {
        ApiService {
            client: Client::new(),
            base_url: base_url.to_string(),
            token,
        }
    }

    async fn get_auth_header(&self) -> Result<HeaderMap, Box<dyn Error>> {
        if let Some(ref token) = self.token {
            let mut headers = HeaderMap::new();
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", token))?,
            );
            Ok(headers)
        } else {
            Err("Authentication token is missing".into())
        }
    }

    pub async fn kick_off_secret_server(
        &self,
        input: server::GetKeysInput,
    ) -> Result<(), Box<dyn Error>> {
        let server = SecretsServer::new(
            self.base_url.clone(),
            self.token.clone().unwrap_or_default(),
        );

        let handle = server.run(input);

        if let Err(e) = handle.await {
            error!("Secrets server failed to run: {}", e);
        }
        Ok(())
    }

    pub async fn get_health(&self) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/health", self.base_url);
        let response = self.client.get(&url).send().await?;

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

        let response = self.client.post(&url).json(&payload).send().await?;

        match response.status() {
            reqwest::StatusCode::CREATED => Ok(()),
            reqwest::StatusCode::BAD_REQUEST | reqwest::StatusCode::CONFLICT => {
                let err_msg: Value = response.json().await?;
                Err(format!("Failed to create account: {:?}", err_msg).into())
            }
            _ => Err(format!("Failed to create account: {}", response.status()).into()),
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
            Ok(response.json().await?)
        } else {
            let error_msg: Value = response
                .json()
                .await
                .unwrap_or_else(|_| json!({ "error": "Unknown error" }));
            Err(format!("Authentication failed: {:?}", error_msg).into())
        }
    }

    pub async fn create_project(
        &self,
        project_input: ProjectInput,
    ) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/projects/create", self.base_url);
        let headers = self.get_auth_header().await?;

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .json(&project_input)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::CREATED {
            Ok(response.json().await?)
        } else {
            let error_msg: Value = response.json().await?;
            Err(format!("Failed to create project: {:?}", error_msg).into())
        }
    }

    pub async fn add_allowed_user(
        &self,
        allowed_user_input: AllowUserInput,
    ) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/projects/allow", self.base_url);
        let headers = self.get_auth_header().await?;

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .json(&allowed_user_input)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::CREATED {
            Ok(response.json().await?)
        } else {
            let error_msg: Value = response.json().await?;
            Err(format!("Failed to add allowed user: {:?}", error_msg).into())
        }
    }

    pub async fn generate_qrcode_file(&self, project_name: String) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/projects/getQRCode/{}", self.base_url, project_name);
        let headers = self.get_auth_header().await?;

        let response = self.client.get(&url).headers(headers).send().await?;
        let filename = format!("{}.png", project_name);

        if response.status() == reqwest::StatusCode::OK {
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(filename)?;

            let content = response.bytes().await?;
            copy(&mut content.as_ref(), &mut file)?;

            Ok(())
        } else {
            let error_msg: Value = response.json().await?;
            Err(format!("Failed to generate QR code: {:?}", error_msg).into())
        }
    }

    pub async fn build_project(&self, input: GetKeysInput) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/projects/getkeys", self.base_url);
        let headers = self.get_auth_header().await?;

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .json(&input)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::OK {
            Ok(response.json().await?)
        } else {
            let error_msg: Value = response.json().await?;
            Err(format!("Failed to build project: {:?}", error_msg).into())
        }
    }

    pub async fn get_all_projects(&self) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/projects/all", self.base_url);
        let headers = self.get_auth_header().await?;

        let response = self.client.get(&url).headers(headers).send().await?;

        if response.status() == reqwest::StatusCode::OK {
            Ok(response.json().await?)
        } else {
            let error_msg: Value = response.json().await?;
            Err(format!("Failed to fetch projects: {:?}", error_msg).into())
        }
    }
}
