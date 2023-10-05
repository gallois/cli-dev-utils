use std::fmt::{Display, Formatter};

use base64::{engine::general_purpose, DecodeError, Engine as _};
use strum_macros::{EnumIter, EnumString, EnumVariantNames};

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum B64Action {
    Encode,
    Decode,
}

#[derive(Debug, PartialEq)]
pub struct B64Error {
    error: DecodeError,
    pub message: String,
}

impl Display for B64Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while decoding base64: {}", self.message)
    }
}

pub fn encode(data: &str) -> String {
    general_purpose::STANDARD_NO_PAD.encode(data)
}

pub fn decode(data: &str) -> Result<String, B64Error> {
    match general_purpose::STANDARD_NO_PAD.decode(data) {
        Ok(decoded) => {
            let result = String::from_utf8(decoded)
                .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
                .unwrap();
            Ok(result)
        }
        Err(e) => Err(B64Error {
            error: e.clone(),
            message: e.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(
            encode("https://theworkoutcalculator.com/"),
            "aHR0cHM6Ly90aGV3b3Jrb3V0Y2FsY3VsYXRvci5jb20v"
        );
    }

    #[test]
    fn test_decode() {
        assert_eq!(
            decode("aHR0cHM6Ly90aGV3b3Jrb3V0Y2FsY3VsYXRvci5jb20v"),
            Ok("https://theworkoutcalculator.com/".to_string())
        )
    }
}
