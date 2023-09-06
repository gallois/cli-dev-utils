use std::str::FromStr;

use chrono::{TimeZone, Utc};
use strum_macros::{EnumIter, EnumString, EnumVariantNames};

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum DateTimeFormat {
    Iso8601,
    Rfc3339,
    Epoch,
    Unix,
}

#[derive(Debug)]
pub struct DateTimeError {
    pub message: String,
}

pub fn convert(from: &str, to: &str, content: &str) -> Result<String, DateTimeError> {
    let from_dt = match DateTimeFormat::from_str(from) {
        Ok(dtf) => match dtf {
            DateTimeFormat::Epoch => {
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
            DateTimeFormat::Epoch => Ok(from_dt.timestamp().to_string()),
            DateTimeFormat::Iso8601 => Ok(from_dt.to_rfc3339()),
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

    // TODO add tests
}
