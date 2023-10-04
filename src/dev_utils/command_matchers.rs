use crate::{dev_utils, Cli, HashArgs, URLArgs};

use super::{hash::HashType, url::UrlAction, CliError};

pub fn hash(hash_args: HashArgs, cli_args: Cli) -> Result<String, CliError> {
    let hash_type = match <HashType as std::str::FromStr>::from_str(&hash_args.hash_type) {
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
    let action = match <UrlAction as std::str::FromStr>::from_str(&url_encode_args.action) {
        Ok(t) => t,
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
