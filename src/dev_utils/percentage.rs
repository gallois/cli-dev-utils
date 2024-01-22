use strum_macros::{EnumIter, EnumString, EnumVariantNames};

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum PercentageAction {
    To,
    Of,
    Change,
}

pub fn to(from_number: f64, to_number: f64, precision: u8) -> Result<String, String> {
    Ok(format!(
        "{value:.precision$}%",
        value = (to_number / from_number) * 100.0,
        precision = precision as usize,
    ))
}

pub fn of(percentage: f32, of_number: f64, precision: u8) -> Result<String, String> {
    Ok(format!(
        "{value:.precision$}",
        value = (percentage as f64 / 100.0) * of_number,
        precision = precision as usize
    ))
}

pub fn change(from_number: f64, to_number: f64, precision: u8) -> Result<String, String> {
    Ok(format!(
        "{value:.precision$}%",
        value = ((to_number - from_number) / from_number.abs()) * 100.0,
        precision = precision as usize,
    ))
}

#[cfg(test)]
#[path = "./percentage_test.rs"]
mod percentage_test;
