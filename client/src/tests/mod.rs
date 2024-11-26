use crate::decoder::decoder::decode_qr_code;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_qr_code() {
        let path = Path::new("/home/kekeli/Repos/shinobi/client/src/tests/shinobi.png");
        let result = decode_qr_code(path);
        println!("Result: {:?}", result);
        assert!(result.is_ok());
    }

    // #[test]
    // fn test_decoded_data() {
    //     let data = DecodedData::new(
    //         String::from("https:://github.com/"),
    //         vec![String::from("echo 'Hello, World!'")],
    //         String::from("123456"),
    //     );
    //     let result = data.run_commands();
    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn test_decoded_data_clone_repo() {
    //     let data = DecodedData::new(
    //         String::from("https://github.com/db-keli/db-keli"),
    //         vec![String::from("echo 'Hello, World!'")],
    //         String::from("123456"),
    //     );
    //     let result = data.clone_repo();
    //     assert!(result.is_ok());
    // }
}
