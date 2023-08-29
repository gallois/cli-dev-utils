use std::string::FromUtf8Error;

use strum_macros::{EnumIter, EnumVariantNames};
use urlencoding;

#[derive(EnumIter, EnumVariantNames)]
pub enum UrlAction {
    Encode,
    Decode,
}

pub fn encode(url: &str) -> String {
    urlencoding::encode(url).into_owned()
}

pub fn decode(url: &str) -> Result<String, FromUtf8Error> {
    match urlencoding::decode(url) {
        Ok(decoded) => Ok(decoded.into_owned()),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(
            encode("https://theworkoutcalculator.com/"),
            "https%3A%2F%2Ftheworkoutcalculator.com%2F"
        );
    }

    #[test]
    fn test_decode() {
        assert_eq!(
            decode("https%3A%2F%2Ftheworkoutcalculator.com%2F"),
            Ok("https://theworkoutcalculator.com/".to_string())
        );
    }
}
