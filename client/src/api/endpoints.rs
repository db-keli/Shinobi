use crate::api::client::ApiService;
use std::error::Error;

pub async fn check_health(api_service: &ApiService) -> Result<(), Box<dyn Error>> {
    let health_status = api_service.get_health().await?;
    println!("Health Status: {}", health_status);
    Ok(())
}

pub async fn create_account(
    api_service: &ApiService,
    name: &str,
    email: &str,
) -> Result<(), Box<dyn Error>> {
    api_service.create_account(name, email)?;
    println!("Account created successfully");
    Ok(())
}
