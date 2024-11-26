// src/decoder.rs
use image::ImageReader;
use quircs::Quirc;
use std::path::Path;

use crate::errors::errors::ImageReadError;

pub fn decode_qr_code(path: &Path) -> Result<(), ImageReadError> {
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
                println!("Decoded: {:?}", std::str::from_utf8(&data.payload).unwrap());
            }
            Err(err) => {
                return Err(ImageReadError::ExtractError(err));
            }
        }
    }

    Ok(())
}
