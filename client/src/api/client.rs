use crate::api::schemas::{
    AllowUserInput, AuthRequest, AuthResponse, CreateUserInput, GetKeysInput, ProjectInput,
};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{copy, prelude::*};

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

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token))?,
        );

        let response = self.client.get(&url).headers(headers).send().await?;

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
            reqwest::StatusCode::CREATED => Ok(()),
            reqwest::StatusCode::BAD_REQUEST => {
                let err_msg: serde_json::Value = response.json().await?;
                Err("Failed to create account: bad request".into())
            }
            reqwest::StatusCode::CONFLICT => {
                let err_msg: serde_json::Value = response.json().await?;
                Err("Failed to create account: user already exists".into())
            }
            _ => {
                let err_msg: serde_json::Value = response.json().await?;
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
            Ok(auth_response)
        } else if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            Err("Invalid credentials".into())
        } else {
            let error_msg: serde_json::Value = response.json().await?;
            Err("Authentication failed".into())
        }
    }

    pub async fn create_project(
        &self,
        project_input: ProjectInput,
    ) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/projects/create", self.base_url);

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token))?,
        );

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .json(&project_input)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::CREATED {
            let project: Value = response.json().await?;
            Ok(project)
        } else {
            let error_msg: Value = response.json().await?;
            Err(format!("{:?}", error_msg).into())
        }
    }

    pub async fn add_allowed_user(
        &self,
        allowed_user_input: AllowUserInput,
    ) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/projects/allow", self.base_url);

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token))?,
        );

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .json(&allowed_user_input)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::CREATED {
            let project: Value = response.json().await?;
            Ok(project)
        } else {
            let error_msg: Value = response.json().await?;
            Err(format!("{:?}", error_msg).into())
        }
    }

    pub async fn generate_qrcode_file(&self, project_name: String) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/projects/getQRCode/{}", self.base_url, project_name);

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token))?,
        );

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
            Err(format!("{:?}", error_msg).into())
        }
    }

    pub async fn build_project(
        &self,
        input: GetKeysInput,
    ) -> Result<serde_json::Value, Box<dyn Error>> {
        let url = format!("{}/projects/getkeys", self.base_url);

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token))?,
        );

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .json(&input)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::OK {
            let project: Value = response.json().await?;
            Ok(project)
        } else {
            let error_msg: Value = response.json().await?;
            Err(format!("{:?}", error_msg).into())
        }
    }

    pub async fn get_all_projects(&self) -> Result<serde_json::Value, Box<dyn Error>> {
        let url = format!("{}/projects/all", self.base_url);

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token))?,
        );

        let response = self.client.get(&url).headers(headers).send().await?;

        if response.status() == reqwest::StatusCode::OK {
            let projects: Value = response.json().await?;
            Ok(projects)
        } else {
            let error_msg: Value = response.json().await?;
            Err(format!("{:?}", error_msg).into())
        }
    }
}
