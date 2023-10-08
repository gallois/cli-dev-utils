use crate::{dev_utils, B64Args, Cli, ConversionArgs, DateTimeArgs, HashArgs, URLArgs, DateArgs};
use std::str::FromStr;
use crate::dev_utils::date::DateAction;

use super::{base64::B64Action, convert::Conversion, hash::HashType, url::UrlAction, CliError};

pub fn hash(hash_args: HashArgs, cli_args: Cli) -> Result<String, CliError> {
    let hash_type = match <HashType as FromStr>::from_str(&hash_args.hash_type) {
        Ok(t) => t,
        Err(_) => {
            return Err(CliError::InvalidArgs(format!(
                "Invalid hash type. Valid hash types are: {}",
                dev_utils::enum_variants::<HashType>()
            )));
        }
    };

    let content = dev_utils::get_content(hash_args.content, cli_args.editor)?;
    let content_str = content.as_str();

    match hash_type {
        HashType::Md5 => Ok(dev_utils::hash::md5(content_str)),
        HashType::Sha256 => Ok(dev_utils::hash::sha256(content_str)),
        HashType::Sha512 => Ok(dev_utils::hash::sha512(content_str)),
    }
}

pub fn url(url_encode_args: URLArgs, cli_args: Cli) -> Result<String, CliError> {
    let action = match <UrlAction as FromStr>::from_str(&url_encode_args.action) {
        Ok(a) => a,
        Err(_) => {
            return Err(CliError::InvalidArgs(format!(
                "Invalid action. Valid actions are: {}",
                dev_utils::enum_variants::<UrlAction>()
            )));
        }
    };

    let url = dev_utils::get_content(url_encode_args.url, cli_args.editor)?;
    let url_str = url.as_str();

    match action {
        UrlAction::Encode => Ok(dev_utils::url::encode(url_str)),
        UrlAction::Decode => match dev_utils::url::decode(url_str) {
            Ok(decoded) => Ok(decoded),
            Err(e) => Err(CliError::UrlError(format!(
                "Error while decoding url: {}",
                e
            ))),
        },
    }
}

pub fn base64(b64_encode_args: B64Args, cli_args: Cli) -> Result<String, CliError> {
    let action = match <B64Action as FromStr>::from_str(&b64_encode_args.action) {
        Ok(a) => a,
        Err(_) => {
            return Err(CliError::InvalidArgs(format!(
                "Invalid action. Valid actions are: {}",
                dev_utils::enum_variants::<B64Action>()
            )));
        }
    };

    let data = dev_utils::get_content(b64_encode_args.data, cli_args.editor)?;
    let data_str = data.as_str();

    match action {
        B64Action::Encode => Ok(dev_utils::base64::encode(data_str)),
        B64Action::Decode => match dev_utils::base64::decode(data_str) {
            Ok(decoded) => Ok(decoded),
            Err(e) => Err(CliError::B64Error(e)),
        },
    }
}

pub fn conversion(convert_args: ConversionArgs, cli_args: Cli) -> Result<String, CliError> {
    let action = match Conversion::from_str(&convert_args.action) {
        Ok(a) => a,
        Err(_) => {
            return Err(CliError::InvalidArgs(format!(
                "Invalid conversion. Valid actions are: {}",
                dev_utils::enum_variants::<Conversion>()
            )));
        }
    };
    let content = dev_utils::get_content(convert_args.content, cli_args.editor)?;
    let content_str = content.as_str();

    match action {
        Conversion::Json2Csv => match dev_utils::convert::json2csv(content_str) {
            Ok(csv) => Ok(csv),
            Err(e) => Err(CliError::ConversionError(e)),
        },
        Conversion::Json2Yaml => match dev_utils::convert::json2yaml(content_str) {
            Ok(yaml) => Ok(yaml),
            Err(e) => Err(CliError::ConversionError(e)),
        },
        Conversion::Csv2Tsv => Ok(dev_utils::convert::csv2tsv(content_str)),
        Conversion::String2Hex => Ok(dev_utils::convert::string2hex(content_str)),
        Conversion::Hex2String => match dev_utils::convert::hex2string(content_str) {
            Ok(data) => Ok(data),
            Err(e) => Err(CliError::ConversionError(e)),
        },
        Conversion::Text2Nato => Ok(dev_utils::convert::text2nato(content_str)),
        Conversion::Slugify => Ok(dev_utils::convert::slugify(content_str)),
        Conversion::Celsius2Fahrenheit | Conversion::C2F => {
            match dev_utils::convert::celsius2fahrenheit(content_str) {
                Ok(data) => Ok(data.to_string()),
                Err(e) => Err(CliError::ConversionError(e)),
            }
        }
        Conversion::Fahrenheit2Celsius | Conversion::F2C => {
            match dev_utils::convert::fahrenheit2celsius(content_str) {
                Ok(data) => Ok(data.to_string()),
                Err(e) => Err(CliError::ConversionError(e)),
            }
        }
        Conversion::Celsius2Kelvin | Conversion::C2K => {
            match dev_utils::convert::celsius2kelvin(content_str) {
                Ok(data) => Ok(data.to_string()),
                Err(e) => Err(CliError::ConversionError(e)),
            }
        }
        Conversion::Kelvin2Celsius | Conversion::K2C => {
            match dev_utils::convert::kelvin2celsius(content_str) {
                Ok(data) => Ok(data.to_string()),
                Err(e) => Err(CliError::ConversionError(e)),
            }
        }
        Conversion::Fahrenheit2Kelvin | Conversion::F2K => {
            match dev_utils::convert::fahrenheit2kelvin(content_str) {
                Ok(data) => Ok(data.to_string()),
                Err(e) => Err(CliError::ConversionError(e)),
            }
        }
        Conversion::Kelvin2Fahrenheit | Conversion::K2F => {
            match dev_utils::convert::kelvin2fahrenheit(content_str) {
                Ok(data) => Ok(data.to_string()),
                Err(e) => Err(CliError::ConversionError(e)),
            }
        }
    }
}

pub fn date_time(date_time_args: DateTimeArgs, args: Cli) -> Result<String, CliError> {
    let content = dev_utils::get_content(date_time_args.content, args.editor)?;
    let content_str = content.as_str();
    match dev_utils::datetime::convert(&date_time_args.from, &date_time_args.to, content_str) {
        Ok(result) => Ok(result),
        Err(e) => Err(CliError::DateTimeError(e)),
    }
}

pub fn date(date_args: DateArgs, cli_args: Cli) -> Result<String, CliError> {
    let action = match DateAction::from_str(&date_args.action) {
        Ok(a) => a,
        Err(_) => {
            return Err(CliError::InvalidArgs(format!(
                "Invalid action. Valid actions are: {}",
                dev_utils::enum_variants::<DateAction>()
            )));
        }
    };
    let content = dev_utils::get_content(date_args.content, cli_args.editor)?;
    let content_str = content.as_str();

    match action {
        DateAction::Delta => match dev_utils::date::delta(content_str, -1) {
            Ok(result) => Ok(result),
            Err(e) => return Err(CliError::DateError(e)),
        },
    }
}
