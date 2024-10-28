mod api;
mod cli;
mod service;

use api::client::ApiService;
use cli::args::build_cli;
use service::locator::ServiceLocator;

fn main() {
    // Build the command-line interface using Clap
    let matches = build_cli().get_matches();

    // Create the service locator and register services
    let mut service_locator = ServiceLocator::new();
    let api_service = ApiService::new(
        "https://shinobi.up.railway.app/v1",
        "DU4SSHOV3W3VCHWZXZQFCLWWPE",
    );
    service_locator.register(api_service);

    // Handle CLI commands and pass the service locator
    crate::cli::commands::handle_commands(matches, &service_locator);
}
