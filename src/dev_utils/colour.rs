use strum_macros::EnumString;
use strum_macros::{EnumIter, EnumVariantNames};
use colors_transform::{Color, Rgb};

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum Colour {
    Hex2Rgb,
    Hex2Hsl,
    Rgb2Hex,
}

#[derive(Debug)]
pub enum ColourConversionError {
    Hex2Rgb(String),
    Hex2Hsl(String),
    Rgb2Hex(String),
}

pub fn hex2rgb(data: &str) -> Result<String, ColourConversionError> {
    let rgb = match Rgb::from_hex_str(data) {
        Ok(c) => c,
        Err(e) => return Err(ColourConversionError::Hex2Rgb(e.message)),
    };

    Ok(rgb.to_css_string())
}

pub fn hex2hsl(data: &str) -> Result<String, ColourConversionError> {
    let hsl = match Rgb::from_hex_str(data) {
        Ok(c) => c.to_hsl(),
        Err(e) => return Err(ColourConversionError::Hex2Hsl(e.message)),
    };

    Ok(hsl.to_css_string())
}

pub fn rgb2hex(data: &str) -> Result<String, ColourConversionError> {
    let rgb = match data.parse::<Rgb>() {
        Ok(c) => c,
        Err(e) => return Err(ColourConversionError::Rgb2Hex(e.message)),
    };

    Ok(rgb.to_css_hex_string())
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

    #[test]
    fn test_hex2hsl() {
        let result = hex2hsl("#1EA54C");
        match result {
            Ok(s) => assert_eq!(s, "hsl(140,69%,38%)"),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_rgb2hex() {
        let result = rgb2hex("rgb(30,165,76)");
        match result {
            Ok(s) => assert_eq!(s, "#1ea54c"),
            Err(e) => panic!("{:#?}", e),
        }
    }
}
