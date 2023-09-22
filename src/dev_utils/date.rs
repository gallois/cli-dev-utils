use strum_macros::EnumString;
use strum_macros::{EnumIter, EnumVariantNames};
use hourglass::{Deltatime, InputError, Timespec, Timezone, TzError};
use regex::{Error, Regex};

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum DateAction {
    Delta,
}

#[derive(Debug)]
pub enum DateError {
    Tz(TzError),
    Input(InputError),
    Regex(Error),
    Capture,
    Parse,
    Format,
}

// FIXME replace this whole thing with Chrono
pub fn delta(content: &str, current_time: i64) -> Result<String, DateError> {
    let current_tz = match Timezone::local() {
        Ok(tz) => tz,
        Err(e) => return Err(DateError::Tz(e)),
    };
    let now = if current_time == -1 {
        current_tz.now()
    } else {
        let timespec = match Timespec::unix(current_time, 0){
            Ok(t) => t,
            Err(e) => return Err(DateError::Input(e)),
        };
        timespec.to_datetime(&current_tz)
    };

    let pattern = match Regex::new("^(?i)(?<sign>-?)(?<value>[0-9]+)(?<unit>[a-z]+)$") {
        Ok(p) => p,
        Err(e) => return Err(DateError::Regex(e)),
    };

    let captures = match pattern.captures(content) {
        Some(c) => c,
        None => return Err(DateError::Capture),
    };

    let multiplier = match &captures["unit"] {
        // That's oversimplifying, isn't it?
        "d" => 1,
        "m" => 30,
        "y" => 365,
        _ => return Err(DateError::Parse),
    };
    let sign = if captures["sign"].is_empty() { 1 } else { -1 };
    let value = match captures["value"].parse::<i64>() {
        Ok(v) => v,
        Err(_) => return Err(DateError::Parse),
    };
    let offset = sign * value * multiplier;

    let result = now + Deltatime::days(offset);

    match result.format("%Y-%m-%d") {
        Ok(s) => Ok(s),
        Err(_) => Err(DateError::Format),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta() {
        let result = delta("1d", 0);
        match result {
            Ok(s) => assert_eq!(s, "1"),
            Err(e) => panic!("{:#?}", e),
        }
    }
}
