use crate::api::client::ApiService;
use crate::api::endpoints::{authenticate, check_health, create_account, create_project};
use crate::api::schemas::ProjectInput;
use crate::service::ServiceLocator;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use clap::ArgMatches;
use std::collections::HashMap;
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

fn prompt_for_map() -> HashMap<String, String> {
    let mut map = HashMap::new();

    loop {
        // Ask for key
        print!("Enter key name (or type 'done' to finish): ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed
        let mut key = String::new();
        io::stdin().read_line(&mut key).unwrap();
        let key = key.trim().to_string();

        if key == "done" {
            break;
        }

        // Ask for value
        print!("Enter value for '{}': ", key);
        io::stdout().flush().unwrap();
        let mut value = String::new();
        io::stdin().read_line(&mut value).unwrap();
        let value = value.trim().to_string();

        // Insert the key-value pair into the map
        map.insert(key, value);
    }

    map
}

fn prompt_for_build_commands() -> Vec<String> {
    let mut vec = Vec::new();

    loop {
        print!("Enter a build command (or type 'done' to finish): ");
        println!("example build command: 'cargo build");
        println!("Make sure commands are in order");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        if input == "done" {
            break;
        }

        vec.push(input);
    }

    vec
}

fn prompt_for_datetime() -> DateTime<Utc> {
    loop {
        print!(
            "Enter a date and time for expiry (in UTC, format YYYY-MM-DDTHH:MM:SS or YYYY-MM-DD): "
        );
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Try to parse the input as a full date-time first (YYYY-MM-DDTHH:MM:SS)
        if let Ok(parsed_date) = Utc.datetime_from_str(input, "%Y-%m-%dT%H:%M:%S") {
            return parsed_date;
        }

        // Try to parse the input as a date-only (YYYY-MM-DD), and assume time as 00:00:00
        if let Ok(parsed_date) = NaiveDate::parse_from_str(input, "%Y-%m-%d") {
            return Utc.from_utc_date(&parsed_date).and_hms(0, 0, 0);
        }

        println!("Invalid date-time format. Please try again.");
    }
}
