use colors_transform::{Color, Hsl, Rgb};
use strum_macros::EnumString;
use strum_macros::{EnumIter, EnumVariantNames};

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum Colour {
    Hex2Rgb,
    Hex2Hsl,
    Rgb2Hex,
    Hsl2Hex,
}

#[derive(Debug)]
pub enum ColourConversionError {
    Hex2Rgb(String),
    Hex2Hsl(String),
    Rgb2Hex(String),
    Hsl2Hex(String),
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

pub fn hsl2hex(data: &str) -> Result<String, ColourConversionError> {
    let hsl = match data.parse::<Hsl>() {
        Ok(c) => c,
        Err(e) => return Err(ColourConversionError::Hsl2Hex(e.message)),
    };

    Ok(hsl.to_rgb().to_css_hex_string())
}

#[cfg(test)]
#[path = "./colour_test.rs"]
mod colour_test;
