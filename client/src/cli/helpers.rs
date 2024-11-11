use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use std::collections::HashMap;
use std::io::{self, Write};

pub fn prompt_user_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

pub fn prompt_for_map() -> HashMap<String, String> {
    let mut map = HashMap::new();

    loop {
        // Ask for key
        print!("Enter key name (or type 'done' to finish): ");
        io::stdout().flush().unwrap();
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

pub fn prompt_for_build_commands() -> Vec<String> {
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

pub fn prompt_for_datetime() -> DateTime<Utc> {
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
