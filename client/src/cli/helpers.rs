use crate::api::schemas::TokenData;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use ring::aead;
use ring::aead::{Aad, BoundKey, LessSafeKey, Nonce, UnboundKey};
use ring::rand::{SecureRandom, SystemRandom};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};

const KEY: &[u8; 35] = b"an_example_very_secure_key_32bytes!";

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
        print!("Enter key name (or type 'done' to finish): ");
        io::stdout().flush().unwrap();
        let mut key = String::new();
        io::stdin().read_line(&mut key).unwrap();
        let key = key.trim().to_string();

        if key == "done" {
            break;
        }

        print!("Enter value for '{}': ", key);
        io::stdout().flush().unwrap();
        let mut value = String::new();
        io::stdin().read_line(&mut value).unwrap();
        let value = value.trim().to_string();

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
        io::stdout().flush().unwrap();
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
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if let Ok(parsed_date) = Utc.datetime_from_str(input, "%Y-%m-%dT%H:%M:%S") {
            return parsed_date;
        }

        if let Ok(parsed_date) = NaiveDate::parse_from_str(input, "%Y-%m-%d") {
            return Utc.from_utc_date(&parsed_date).and_hms(0, 0, 0);
        }

        println!("Invalid date-time format. Please try again.");
    }
}

pub fn save_token_toml(token_data: &TokenData) -> std::io::Result<()> {
    let current_dir = std::env::current_dir()?;
    let hidden_dir = current_dir.join(".shinobi");
    if !hidden_dir.exists() {
        fs::create_dir_all(&hidden_dir)?;
    }

    let file_path = hidden_dir.join("shinobisafe.toml");

    let toml_data = toml::to_string_pretty(token_data).unwrap();

    let mut file = File::create(file_path)?;
    file.write_all(toml_data.as_bytes())?;

    Ok(())
}

fn encrypt(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let key = UnboundKey::new(&aead::AES_256_GCM, KEY).unwrap();
    let less_safe_key = LessSafeKey::new(key);

    let rng = SystemRandom::new();
    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes).unwrap();
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);

    let tag_len = aead::AES_256_GCM.tag_len();
    let mut in_out = data.to_vec();
    in_out.extend_from_slice(&vec![0u8; tag_len]);

    less_safe_key
        .seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
        .unwrap();

    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&in_out);
    Ok(result)
}

/// Decrypts the data with AES-GCM.
fn decrypt(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if data.len() < 12 {
        return Err("Invalid ciphertext length".into());
    }

    let key = UnboundKey::new(&aead::AES_256_GCM, KEY).unwrap();
    let less_safe_key = LessSafeKey::new(key);

    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::assume_unique_for_key(<[u8; 12]>::try_from(nonce_bytes)?);

    let mut in_out = ciphertext.to_vec();
    let plaintext = less_safe_key
        .open_in_place(nonce, Aad::empty(), &mut in_out)
        .unwrap();
    Ok(plaintext.to_vec())
}
