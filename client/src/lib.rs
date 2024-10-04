use image::{ImageError, ImageReader};
use quircs::{DecodeError, ExtractError, Quirc};
use std::fmt;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub enum ImageReadError {
    IoError(io::Error),
    ImgError(ImageError),
    DecodeError(DecodeError),
    ExtractError(ExtractError),
}

impl From<ImageError> for ImageReadError {
    fn from(err: ImageError) -> Self {
        ImageReadError::ImgError(err)
    }
}

impl From<DecodeError> for ImageReadError {
    fn from(err: DecodeError) -> Self {
        ImageReadError::DecodeError(err)
    }
}

impl From<io::Error> for ImageReadError {
    fn from(err: io::Error) -> Self {
        ImageReadError::IoError(err)
    }
}

impl From<ExtractError> for ImageReadError {
    fn from(err: ExtractError) -> Self {
        ImageReadError::ExtractError(err)
    }
}

impl fmt::Display for ImageReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageReadError::IoError(err) => write!(f, "IO error: {}", err),
            ImageReadError::ImgError(err) => write!(f, "Image error: {}", err),
            ImageReadError::DecodeError(err) => write!(f, "Decode error: {}", err),
            ImageReadError::ExtractError(err) => write!(f, "Extract error: {}", err),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_qr_code() {
        let path = Path::new("./tests/qrcode.jpg");
        let result = decode_qr_code(path);
        println!("Result: {:?}", result);
        assert!(result.is_ok());
    }
}
