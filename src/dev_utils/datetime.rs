use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use chrono::{DateTime, LocalResult, TimeZone, Utc};
use strum_macros::{EnumIter, EnumString, EnumVariantNames};

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum DateTimeFormat {
    Iso8601,
    Iso9075,
    Rfc3339,
    Rfc2822,
    Epoch,
    Unix,
}

#[derive(Debug, PartialEq)]
pub struct DateTimeError {
    pub message: String,
}

impl Display for DateTimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
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
                match Utc.timestamp_opt(from_i64, 0) {
                    LocalResult::Single(from_dt) => from_dt.fixed_offset(),
                    _ => {
                        return Err(DateTimeError {
                            message: format!("Cannot parse {} to DateTime", from),
                        })
                    }
                }
            }
            DateTimeFormat::Iso8601 | DateTimeFormat::Rfc3339 => {
                match content.parse::<chrono::DateTime<Utc>>() {
                    Ok(from_dt) => from_dt.fixed_offset(),
                    Err(_) => {
                        return Err(DateTimeError {
                            message: format!("Cannot parse {} to DateTime", from),
                        })
                    }
                }
            }
            DateTimeFormat::Rfc2822 => match DateTime::parse_from_rfc2822(content) {
                Ok(from_dt) => from_dt,
                Err(_) => {
                    return Err(DateTimeError {
                        message: format!("Cannot parse {} to DateTime", from),
                    })
                }
            },
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
            DateTimeFormat::Rfc2822 => Ok(from_dt.to_rfc2822()),
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
#[path = "./datetime_test.rs"]
mod datetime_test;
