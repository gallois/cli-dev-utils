use chrono::LocalResult::Single;
use chrono::{Duration, Months, TimeZone};
use regex::{Error, Regex};
use std::fmt::{Display, Formatter};
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
        chrono::Utc::now()
    } else {
        match chrono::Utc.timestamp_opt(current_time, 0) {
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
#[path = "./date_test.rs"]
mod date_test;
