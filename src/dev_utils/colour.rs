use strum_macros::EnumString;
use strum_macros::{EnumIter, EnumVariantNames};
use colors_transform::{Color, Rgb};

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum Colour {
    Hex2Rgb,
}

#[derive(Debug)]
pub enum ColourConversionError {
    Hex2Rgb(String),
}

pub fn hex2rgb(data: &str) -> Result<String, ColourConversionError> {
    let rgb = match Rgb::from_hex_str(data) {
        Ok(c) => c,
        Err(e) => return Err(ColourConversionError::Hex2Rgb(e.message)),
    };

    Ok(rgb.to_css_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex2rgb() {
        let result = hex2rgb("#1EA54C");
        match result {
            Ok(s) => assert_eq!(s, "rgb(30,165,76)"),
            Err(e) => panic!("{:#?}", e),
        }
    }
}
