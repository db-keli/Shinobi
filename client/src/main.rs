mod api;
mod cli;
mod decoder;
mod errors;
mod service;

use api::client::ApiService;
use api::schemas::TokenData;
use cli::args::build_cli;
use service::locator::ServiceLocator;
use std::fs;

fn main() {
    let matches = build_cli().get_matches();

    let mut service_locator = ServiceLocator::new();
    let token = load_token_toml().unwrap().token;

    let api_service = ApiService::new("https://shinobi.up.railway.app/v1", &token);
    service_locator.register(api_service);

    crate::cli::commands::handle_commands(matches, &service_locator);
}

fn load_token_toml() -> std::io::Result<TokenData> {
    let current_dir = std::env::current_dir()?;
    let file_path = current_dir.join(".shinobi").join("shinobisafe.toml");

    if file_path.exists() {
        let file_content = fs::read_to_string(file_path)?;
        let token_data: TokenData = toml::from_str(&file_content).unwrap();
        Ok(token_data)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Token file not found",
        ))
    }
}
