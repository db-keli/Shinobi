use crate::api::client::ApiService;
use crate::api::endpoints::{
    add_allowed_user, authenticate, check_health, create_account, create_project,
    generate_qrcode_file, kick_off_server,
};
use crate::api::schemas::{AllowUserInput, ProjectInput};
use crate::cli::helpers::{
    prompt_for_build_commands, prompt_for_datetime, prompt_for_map, prompt_user_input,
    save_token_toml,
};
use crate::decoder::decoder::decode_qr_code;
use crate::service::ServiceLocator;
use shinobi_secrets_server::server::server;

use clap::ArgMatches;
use std::path::Path;
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

            let token =
                Runtime::new()
                    .unwrap()
                    .block_on(authenticate(api_service, &email, &password));

            match token {
                Ok(auth_response) => {
                    let token_data = auth_response.authentication_token;

                    if let Err(_) = save_token_toml(&token_data) {
                        eprintln!("Failed to save token");
                    }
                }

                Err(_) => eprintln!("Failed to authenticate"),
            }
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

    if let Some(_) = matches.subcommand_matches("allow") {
        if let Some(api_service) = service_locator.get::<ApiService>() {
            let user_email = prompt_user_input("Enter user email:\n");
            let project_name = prompt_user_input(&format!(
                "Enter project name to allow for {}: \n",
                user_email
            ));

            let input = AllowUserInput {
                user_email,
                project_name,
            };

            let _ = Runtime::new()
                .unwrap()
                .block_on(add_allowed_user(api_service, input));
        }
    }

    if let Some(_) = matches.subcommand_matches("qrcode") {
        if let Some(api_service) = service_locator.get::<ApiService>() {
            let project_name = prompt_user_input("Enter project name: \n");

            let _ = Runtime::new()
                .unwrap()
                .block_on(generate_qrcode_file(api_service, project_name));
        }
    }

    if let Some(matches) = matches.subcommand_matches("getkeys") {
        if let Some(api_service) = service_locator.get::<ApiService>() {
            let project_name = matches
                .get_one::<String>("project")
                .cloned()
                .unwrap_or_default();

            let qrcode_file = matches
                .get_one::<String>("qrcode")
                .cloned()
                .unwrap_or_default();

            let token = decode_qr_code(Path::new(&qrcode_file)).unwrap().keys_token;

            let input = server::GetKeysInput {
                project_name: project_name.clone(),
                token: token.clone(),
            };

            let _ = Runtime::new()
                .unwrap()
                .block_on(kick_off_server(api_service, input));
        }

        if let Some(_) = matches.subcommand_matches("all_projects") {
            if let Some(api_service) = service_locator.get::<ApiService>() {
                let _ = Runtime::new()
                    .unwrap()
                    .block_on(api_service.get_all_projects());
            }
        }
    }
}
