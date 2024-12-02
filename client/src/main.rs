mod api;
mod cli;
mod decoder;
mod errors;
mod service;

use api::client::ApiService;
use api::schemas::TokenData;
use cli::args::build_cli;
use service::locator::ServiceLocator;
use std::{fs, path::PathBuf};

fn main() {
    let matches = build_cli().get_matches();

    let mut service_locator = ServiceLocator::new();

    let token = load_token_toml().map(|data| data.token);

    let api_service = ApiService::new("https://shinobi.up.railway.app/v1", token);
    service_locator.register(api_service);

    crate::cli::commands::handle_commands(matches, &service_locator);
}

fn load_token_toml() -> Option<TokenData> {
    let current_dir = std::env::current_dir().ok()?;

    let file_path: PathBuf = current_dir.join(".shinobi").join("shinobisafe.toml");

    if file_path.exists() {
        if let Ok(file_content) = fs::read_to_string(&file_path) {
            if let Ok(token_data) = toml::from_str(&file_content) {
                return Some(token_data);
            } else {
                eprintln!("Warning: Failed to parse TOML file. Proceeding without token.");
            }
        } else {
            eprintln!("Warning: Failed to read token file. Proceeding without token.");
        }
    } else {
        eprintln!("Info: Token file not found. Proceeding without token.");
    }

    None
}
