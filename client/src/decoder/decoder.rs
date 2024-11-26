// src/decoder.rs
use image::ImageReader;
use quircs::Quirc;
use serde::{de::Error, Deserialize};
use std::path::Path;

use crate::errors::errors::ImageReadError;

#[derive(Debug, Deserialize)]
pub struct Project {
    pub id: u32,
    pub name: String,
    pub user_id: u32,
    pub project_url: String,
    pub build_commands: Vec<String>,
    pub keys_token: String,
    pub expire_at: String,
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
    #[serde(rename = "UpdatedAt")]
    pub updated_at: String,
}

pub fn decode_qr_code(path: &Path) -> Result<Project, ImageReadError> {
    // Load the image
    let img = ImageReader::open(path)
        .map_err(ImageReadError::IoError)?
        .decode()
        .map_err(ImageReadError::ImgError)?
        .to_luma8();

    let mut decoder = Quirc::default();
    let codes = decoder.identify(img.width() as usize, img.height() as usize, &img);

    for code in codes {
        match code {
            Ok(code) => {
                let data = code.decode().map_err(ImageReadError::DecodeError)?;
                let payload = std::str::from_utf8(&data.payload).map_err(|_| {
                    ImageReadError::JsonError(serde_json::Error::custom("UTF-8 parsing failed"))
                })?;

                // Deserialize the JSON payload into the `Project` struct
                let project: Project =
                    serde_json::from_str(payload).map_err(ImageReadError::JsonError)?;
                return Ok(project);
            }
            Err(err) => {
                return Err(ImageReadError::ExtractError(err));
            }
        }
    }

    Err(ImageReadError::JsonError(serde_json::Error::custom(
        "No valid QR code found",
    )))
}
