use crate::api::client::ApiService;
use crate::api::endpoints::{authenticate, check_health, create_account, create_project};
use crate::api::schemas::ProjectInput;
use crate::cli::helpers::{
    prompt_for_build_commands, prompt_for_datetime, prompt_for_map, prompt_user_input,
};
use crate::service::ServiceLocator;
use clap::ArgMatches;
use tokio::runtime::Runtime;

pub fn handle_commands(matches: ArgMatches, service_locator: &ServiceLocator) {
    if let Some(_) = matches.subcommand_matches("health") {
        if let Some(api_service) = service_locator.get::<crate::api::client::ApiService>() {
            let _ = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(check_health(api_service));
        }
    }

    if let Some(_) = matches.subcommand_matches("init") {
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

            let _ = Runtime::new()
                .unwrap()
                .block_on(authenticate(api_service, &email, &password));
        }
    }

    if let Some(_) = matches.subcommand_matches("create_project") {
        if let Some(api_service) = service_locator.get::<ApiService>() {
            let name = prompt_user_input("Enter the project name: ");
            let project_url = prompt_user_input("Enter the project URL: ");
            let build_commands = prompt_for_build_commands();
            let keys = prompt_for_map();
            let expiry = prompt_for_datetime();
            let input = ProjectInput {
                name,
                project_url,
                build_commands,
                keys,
                expire_at: expiry,
            };

            let _ = Runtime::new()
                .unwrap()
                .block_on(create_project(api_service, input));
        }
    }
}
