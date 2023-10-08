use std::fmt::{Display, Formatter};
use chrono::LocalResult::Single;
use chrono::{Duration, Months, TimeZone};
use regex::{Error, Regex};
use strum_macros::EnumString;
use strum_macros::{EnumIter, EnumVariantNames};

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum DateAction {
    Delta,
}

#[derive(Debug)]
pub enum DateError {
    Input,
    Regex(Error),
    Capture,
    Parse,
    InvalidUnit,
}

impl Display for DateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn delta(content: &str, current_time: i64) -> Result<String, DateError> {
    let now = if current_time < 0 {
        chrono::Local::now()
    } else {
        match chrono::Local.timestamp_opt(current_time, 0) {
            Single(t) => t,
            _ => return Err(DateError::Input),
        }
    };
    let pattern = match Regex::new("^(?i)(?<sign>-?)(?<value>[0-9]+)(?<unit>[a-z]+)$") {
        Ok(p) => p,
        Err(e) => return Err(DateError::Regex(e)),
    };

    let captures = match pattern.captures(content) {
        Some(c) => c,
        None => return Err(DateError::Capture),
    };
    let sign = if captures["sign"].is_empty() { 1 } else { -1 };
    let value = match captures["value"].parse::<i64>() {
        Ok(v) => v,
        Err(_) => return Err(DateError::Parse),
    };

    let offset = match &captures["unit"] {
        "d" => now + Duration::days(sign * value),
        "m" => now + Months::new((sign * value) as u32),
        "y" => now + Months::new((12 * sign * value) as u32),
        _ => return Err(DateError::InvalidUnit),
    };

    Ok(offset.format("%Y-%m-%d").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta() {
        let result = delta("1d", 0);
        match result {
            Ok(s) => assert_eq!(s, "1970-01-02"),
            Err(e) => panic!("{:#?}", e),
        }

        let result = delta("1m", 0);
        match result {
            Ok(s) => assert_eq!(s, "1970-02-01"),
            Err(e) => panic!("{:#?}", e),
        }

        let result = delta("-1d", 88000);
        match result {
            Ok(s) => assert_eq!(s, "1970-01-01"),
            Err(e) => panic!("{:#?}", e),
        }
    }
}
