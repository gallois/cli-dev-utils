use crate::dev_utils::date::DateAction;
use crate::{
    dev_utils, B64Args, Cli, ColourArgs, ConversionArgs, DateArgs, DateTimeArgs, GenerateArgs,
    HashArgs, ListArgs, URLArgs,
};
use std::str::FromStr;

use super::colour::Colour;
use super::generate::{GenerateParams, GenerateSubcommands};
use super::list::ListAction;
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
        UrlAction::Parse => match dev_utils::url::parse(url_str) {
            Ok(parsed) => Ok(parsed),
            Err(e) => Err(CliError::UrlError(format!(
                "Error while parsing url: {}",
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
            Err(e) => Err(CliError::DateError(e)),
        },
    }
}

pub fn list(list_args: ListArgs, cli_args: Cli) -> Result<String, CliError> {
    let action = match ListAction::from_str(&list_args.action) {
        Ok(a) => a,
        Err(_) => {
            return Err(CliError::InvalidArgs(format!(
                "Invalid action. Valid actions are: {}",
                dev_utils::enum_variants::<ListAction>()
            )));
        }
    };
    let content = dev_utils::get_content(list_args.content, cli_args.editor)?;
    let content_str = content.as_str();
    let separator = list_args.separator.as_str();

    match action {
        ListAction::Sort => Ok(dev_utils::list::sort(content_str, separator)),
        ListAction::Lowercase => Ok(dev_utils::list::lowercase(content_str, separator)),
        ListAction::Uppercase => Ok(dev_utils::list::uppercase(content_str, separator)),
        ListAction::Capitalise | ListAction::Capitalize => {
            Ok(dev_utils::list::capitalise(content_str, separator))
        }
        ListAction::Reverse => Ok(dev_utils::list::reverse(content_str, separator)),
    }
}

pub fn colour(colour_args: ColourArgs, cli_args: Cli) -> Result<String, CliError> {
    let action = match Colour::from_str(&colour_args.action) {
        Ok(a) => a,
        Err(_) => {
            return Err(CliError::InvalidArgs(format!(
                "Invalid action. Valid actions are: {}",
                dev_utils::enum_variants::<Colour>()
            )));
        }
    };
    let content = dev_utils::get_content(colour_args.content, cli_args.editor)?;
    let content_str = content.as_str();

    match action {
        Colour::Hex2Rgb => match dev_utils::colour::hex2rgb(content_str) {
            Ok(rgb) => Ok(rgb),
            Err(e) => Err(CliError::ColourError(format!(
                "Error while converting hex to rgb: {:#?}",
                e,
            ))),
        },
        Colour::Hex2Hsl => match dev_utils::colour::hex2hsl(content_str) {
            Ok(hsl) => Ok(hsl),
            Err(e) => Err(CliError::ColourError(format!(
                "Error while converting hex to hsl: {:#?}",
                e
            ))),
        },
        Colour::Rgb2Hex => match dev_utils::colour::rgb2hex(content_str) {
            Ok(rgb) => Ok(rgb),
            Err(e) => Err(CliError::ColourError(format!(
                "Error while converting rgb to hex: {:#?}\nFormat should be `rgb(r,g,b)`",
                e
            ))),
        },
        Colour::Hsl2Hex => match dev_utils::colour::hsl2hex(content_str) {
            Ok(hsl) => Ok(hsl),
            Err(e) => Err(CliError::ColourError(format!(
                "Error while converting hsl to hex: {:#?}\nFormat should be `hsl(h,s,l)`",
                e
            ))),
        },
    }
}

pub fn generate(generate_args: GenerateArgs) -> Result<String, CliError> {
    match generate_args.type_ {
        GenerateSubcommands::Token {
            length,
            no_uppercase,
            no_lowercase,
            no_numbers,
            no_symbols,
        } => match dev_utils::generate::token(
            length,
            no_uppercase,
            no_lowercase,
            no_numbers,
            no_symbols,
        ) {
            Ok(token) => Ok(token),
            Err(e) => Err(CliError::GenerateError(e)),
        },
        GenerateSubcommands::Uuid {
            version,
            namespace,
            name,
            node_id,
            count,
        } => {
            let params = GenerateParams {
                version,
                namespace,
                name,
                node_id,
            };

            let count = count.unwrap_or(1);
            let mut result: Vec<String> = vec![];
            for _ in 0..count {
                match dev_utils::generate::uuid(&params) {
                    Ok(uuid) => result.push(uuid),
                    Err(e) => return Err(CliError::GenerateError(e)),
                }
            }
            Ok(result.join("\n"))
        }
        GenerateSubcommands::Ulid { count } => {
            let count = count.unwrap_or(1);
            let mut result = vec![];
            for _ in 0..count {
                result.push(dev_utils::generate::ulid());
            }
            Ok(result.join("\n"))
        }
    }
}
