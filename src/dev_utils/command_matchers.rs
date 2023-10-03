use crate::{dev_utils, Cli, HashArgs};

use super::{hash::HashType, CliError};

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
