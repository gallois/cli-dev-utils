use std::str::FromStr;

use chrono::{TimeZone, Utc};
use strum_macros::{EnumIter, EnumString, EnumVariantNames};

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum DateTimeFormat {
    Iso8601,
    Iso9075,
    Rfc3339,
    Epoch,
    Unix,
}

#[derive(Debug, PartialEq)]
pub struct DateTimeError {
    pub message: String,
}

pub fn convert(from: &str, to: &str, content: &str) -> Result<String, DateTimeError> {
    let from_dt = match DateTimeFormat::from_str(from) {
        Ok(dtf) => match dtf {
            DateTimeFormat::Epoch | DateTimeFormat::Unix => {
                let from_i64 = match content.parse::<i64>() {
                    Ok(from_i64) => from_i64,
                    Err(_) => {
                        return Err(DateTimeError {
                            message: format!("Cannot parse {} to i64", from),
                        })
                    }
                };
                Utc.timestamp_opt(from_i64, 0).unwrap()
            }
            DateTimeFormat::Iso8601 | DateTimeFormat::Rfc3339 => {
                match content.parse::<chrono::DateTime<Utc>>() {
                    Ok(from_dt) => from_dt,
                    Err(_) => {
                        return Err(DateTimeError {
                            message: format!("Cannot parse {} to DateTime", from),
                        })
                    }
                }
            }
            _ => {
                return Err(DateTimeError {
                    message: "`from` not implemented".to_string(),
                })
            }
        },
        Err(_) => {
            return Err(DateTimeError {
                message: format!("Invalid 'from' format: {}", from),
            })
        }
    };

    match DateTimeFormat::from_str(to) {
        Ok(dtf) => match dtf {
            DateTimeFormat::Epoch | DateTimeFormat::Unix => Ok(from_dt.timestamp().to_string()),
            // ISO8601 and RFC3339 are not exactly the same, but it will do for now
            // https://ijmacd.github.io/rfc3339-iso8601/
            DateTimeFormat::Iso8601 | DateTimeFormat::Rfc3339 => Ok(from_dt.to_rfc3339()),
            _ => Err(DateTimeError {
                message: "`to` not implemented".to_string(),
            }),
        },
        Err(_) => Err(DateTimeError {
            message: format!("Invalid 'to' format: {}", to),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epoch_to_iso8601() {
        assert_eq!(
            convert("epoch", "iso8601", "1"),
            Ok("1970-01-01T00:00:01+00:00".to_string())
        );
    }
    #[test]
    fn test_unix_to_iso8601() {
        assert_eq!(
            convert("unix", "iso8601", "1"),
            Ok("1970-01-01T00:00:01+00:00".to_string())
        );
    }
    #[test]
    fn test_epoch_to_rfc3339() {
        assert_eq!(
            convert("epoch", "rfc3339", "1"),
            Ok("1970-01-01T00:00:01+00:00".to_string())
        );
    }
    #[test]
    fn test_unix_to_rfc3339() {
        assert_eq!(
            convert("unix", "rfc3339", "1"),
            Ok("1970-01-01T00:00:01+00:00".to_string())
        );
    }
    #[test]
    fn test_iso8601_to_epoch() {
        assert_eq!(
            convert("iso8601", "epoch", "1970-01-01T00:00:01+00:00"),
            Ok("1".to_string())
        );
    }
    #[test]
    fn test_iso8601_to_unix() {
        assert_eq!(
            convert("iso8601", "unix", "1970-01-01T00:00:01+00:00"),
            Ok("1".to_string())
        );
    }
    #[test]
    fn test_rfc3999_to_epoch() {
        assert_eq!(
            convert("rfc3339", "epoch", "1970-01-01T00:00:01+00:00"),
            Ok("1".to_string())
        );
    }
    #[test]
    fn test_rfc3339_to_unix() {
        assert_eq!(
            convert("rfc3339", "unix", "1970-01-01T00:00:01+00:00"),
            Ok("1".to_string())
        );
    }
}
