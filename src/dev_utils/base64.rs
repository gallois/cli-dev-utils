use base64::{engine::general_purpose, Engine as _};
use strum_macros::{EnumIter, EnumString, EnumVariantNames};

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum B64Action {
    Encode,
    Decode,
}

pub fn encode(data: &str) -> String {
    general_purpose::STANDARD_NO_PAD.encode(data)
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
}
