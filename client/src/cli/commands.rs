use crate::api::client::ApiService;
use crate::api::endpoints::{check_health, create_account};
use crate::service::ServiceLocator;
use clap::ArgMatches;
use std::io::{self, Write};
use tokio::runtime::Runtime;

pub fn handle_commands(matches: ArgMatches, service_locator: &ServiceLocator) {
    if let Some(_) = matches.subcommand_matches("health") {
        if let Some(api_service) = service_locator.get::<crate::api::client::ApiService>() {
            let _ = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(check_health(api_service));
        }
    }

    if let Some(_) = matches.subcommand_matches("create-account") {
        if let Some(api_service) = service_locator.get::<ApiService>() {
            let name = prompt_user_input("Enter your name: ");
            let email = prompt_user_input("Enter your email: ");
            let password = prompt_user_input("Enter your password: ");

            let _ = Runtime::new().unwrap().block_on(create_account(
                api_service,
                &name,
                &email,
                &password,
            ));
        }
    }

    // More command handling logic can be added here
}

fn prompt_user_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt); // Print the prompt message
    io::stdout().flush().unwrap(); // Ensure prompt is printed before input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string() // Return the input, trimmed of newline
}
