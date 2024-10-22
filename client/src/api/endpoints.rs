use crate::api::client::ApiService;
use std::error::Error;

use super::schemas::AuthResponse;

pub async fn check_health(api_service: &ApiService) -> Result<(), Box<dyn Error>> {
    let health_status = api_service.get_health().await?;
    println!("Health Status: {}", health_status);
    Ok(())
}

pub async fn create_account(
    api_service: &ApiService,
    name: &str,
    email: &str,
    password: &str,
) -> Result<(), Box<dyn Error>> {
    match api_service.create_account(name, email, password).await {
        Ok(created_account) => {
            println!("Account created successfully!");
            println!("{:?}", created_account);
            Ok(())
        }
        Err(e) => {
            if e.is::<reqwest::Error>() {
                eprintln!("Failed to communicate with the server: {}", e);
            } else if e.is::<serde_json::Error>() {
                eprintln!("Error parsing the server's response: {}", e);
            } else {
                eprintln!("An error occurred during account creation: {}", e);
            }
            Err(e)
        }
    }
}

pub async fn authenticate(
    api_service: &ApiService,
    email: &str,
    password: &str,
) -> Result<AuthResponse, Box<dyn Error>> {
    match api_service.get_authentication_token(email, password).await {
        Ok(token) => Ok(token),
        Err(e) => {
            if e.is::<reqwest::Error>() {
                eprintln!("Failed to communicate with the server: {}", e);
            } else if e.is::<serde_json::Error>() {
                eprintln!("Error parsing the server's response: {}", e);
            } else {
                eprintln!("An error occurred during account creation: {}", e);
            }
            Err(e)
        }
    }
}
