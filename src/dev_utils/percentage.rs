use strum_macros::{EnumIter, EnumString, EnumVariantNames};

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum PercentageAction {
    To,
}

pub fn to(from_number: f64, to_number: f64, precision: u8) -> Result<String, String> {
    Ok(format!(
        "{value:.precision$}%",
        value = (to_number / from_number) * 100.0,
        precision = precision as usize,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to() {
        assert_eq!(to(100.0, 50.0, 0).unwrap(), "50%");
        assert_eq!(to(150.0, 50.0, 2).unwrap(), "33.33%");
    }
}
