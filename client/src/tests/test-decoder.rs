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
