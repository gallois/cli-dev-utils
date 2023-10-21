use strum_macros::{EnumIter, EnumString, EnumVariantNames};

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum PercentageAction {
    To,
    Of,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to() {
        assert_eq!(to(100.0, 50.0, 0).unwrap(), "50%");
        assert_eq!(to(150.0, 50.0, 2).unwrap(), "33.33%");
    }

    #[test]
    fn test_of() {
        assert_eq!(of(50.0, 100.0, 0).unwrap(), "50");
        assert_eq!(of(33.33, 100.0, 2).unwrap(), "33.33");
        assert_eq!(of(25.0, 200.0, 3).unwrap(), "50.000");
    }
}
