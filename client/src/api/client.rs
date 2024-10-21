use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
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

    pub fn create_account(&self, name: &str, email: &str) -> Result<(), Box<dyn Error>> {
        // Implement the logic to create an account
        Ok(())
    }

    // Additional methods for other endpoints can be added here
}
