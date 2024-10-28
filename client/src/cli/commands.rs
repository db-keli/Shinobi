use crate::api::client::ApiService;
use crate::api::endpoints::{
    add_allowed_user, authenticate, build_project, check_health, create_account, create_project,
    generate_qrcode_file,
};
use crate::api::schemas::{AllowUserInput, GetKeysInput, ProjectInput};
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

    if let Some(_) = matches.subcommand_matches("build") {
        if let Some(api_service) = service_locator.get::<ApiService>() {
            let project_name = prompt_user_input("Enter project Name: ");
            let token = prompt_user_input("enter token: ");

            let input = GetKeysInput {
                project_name,
                token,
            };

            let _ = Runtime::new()
                .unwrap()
                .block_on(build_project(api_service, input));
        }
    }
}
