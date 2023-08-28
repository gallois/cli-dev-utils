use strum_macros::{EnumIter, EnumVariantNames};
use urlencoding;

#[derive(EnumIter, EnumVariantNames)]
pub enum UrlAction {
    Encode,
}

pub fn encode(url: &str) -> String {
    urlencoding::encode(url).into_owned()
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
}
