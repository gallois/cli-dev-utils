use std::fmt::{Display, Formatter};

use clap::{arg, Subcommand};
use rand::Rng;
use strum_macros::{EnumIter, EnumString, EnumVariantNames};

use uuid::Uuid;

#[derive(Clone, EnumIter, EnumString, EnumVariantNames, Subcommand)]
#[strum(serialize_all = "lowercase")]
pub enum GenerateSubcommands {
    Token {
        length: u8,
        #[arg(short = 'u', long)]
        no_uppercase: bool,
        #[arg(short = 'l', long)]
        no_lowercase: bool,
        #[arg(short = 'n', long)]
        no_numbers: bool,
        #[arg(short = 's', long)]
        no_symbols: bool,
    },
    Uuid {
        version: Option<String>,
        #[arg(long)]
        namespace: Option<String>,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        node_id: Option<String>,
    },
}

pub struct GenerateParams {
    pub version: Option<String>,
    pub namespace: Option<String>,
    pub name: Option<String>,
    pub node_id: Option<String>,
}

#[derive(Debug)]
pub enum GenerateError {
    Token(String),
    Uuid(String),
}

impl Display for GenerateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn token(
    length: u8,
    no_uppercase: bool,
    no_lowercase: bool,
    no_numbers: bool,
    no_symbols: bool,
) -> Result<String, GenerateError> {
    if no_uppercase && no_lowercase && no_numbers && no_symbols {
        return Err(GenerateError::Token("Nothing to generate".to_string()));
    }

    let mut uppercase_set = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let mut lowercase_set = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let mut numbers_set = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut symbols_set = vec![
        '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', ']', '{', '}',
        '.', '?', '/',
    ];
    let mut char_set = vec![];
    if !no_uppercase {
        char_set.append(&mut uppercase_set);
    }
    if !no_lowercase {
        char_set.append(&mut lowercase_set);
    }
    if !no_numbers {
        char_set.append(&mut numbers_set);
    }
    if !no_symbols {
        char_set.append(&mut symbols_set);
    }

    let mut rng = rand::thread_rng();
    let mut token = String::new();

    for _ in 0..length {
        token.push(char_set[rng.gen_range(0..char_set.len())]);
    }

    Ok(token)
}

pub fn uuid(params: GenerateParams) -> Result<String, GenerateError> {
    let version = match params.version {
        Some(version) => match version.as_str() {
            "v4" | "4" => 4,
            "v3" | "3" => 3,
            "v5" | "5" => 5,
            "v1" | "1" => 1,
            "v2" | "2" | "v6" | "6" | "v7" | "7" | "v8" | "8" => {
                return Err(GenerateError::Uuid(format!(
                    "Version {} not implemented",
                    version
                )));
            }
            _ => return Err(GenerateError::Uuid("Invalid value for version".to_string())),
        },
        None => 4,
    };

    match version {
        4 => Ok(Uuid::new_v4().to_string()),
        3 | 5 => {
            let namespace = match &params.namespace {
                Some(namespace) => {
                    if namespace.is_empty() {
                        return Err(GenerateError::Uuid("Missing namespace".to_string()));
                    }

                    match namespace.to_ascii_lowercase().as_str() {
                        "dns" => Uuid::NAMESPACE_DNS,
                        "url" => Uuid::NAMESPACE_URL,
                        "oid" => Uuid::NAMESPACE_OID,
                        "x500" => Uuid::NAMESPACE_X500,
                        _ => {
                            return Err(GenerateError::Uuid(format!(
                                "Invalid value for namespace: {}",
                                namespace
                            )))
                        }
                    }
                }
                None => return Err(GenerateError::Uuid("Missing namespace".to_string())),
            };

            let name = match &params.name {
                Some(name) => {
                    if name.is_empty() {
                        return Err(GenerateError::Uuid("Missing name".to_string()));
                    }

                    name.as_bytes()
                }
                None => return Err(GenerateError::Uuid("Missing name".to_string())),
            };

            match version {
                3 => Ok(Uuid::new_v3(&namespace, name).to_string()),
                5 => Ok(Uuid::new_v5(&namespace, name).to_string()),
                _ => unreachable!(),
            }
        }
        1 => {
            let node_id_slice: &[u8; 6] = match &params.node_id {
                Some(node_id) => {
                    if node_id.is_empty() {
                        return Err(GenerateError::Uuid("Missing node_id".to_string()));
                    }
                    match node_id.as_bytes()[..6].try_into() {
                        Ok(slice) => slice,
                        Err(_) => {
                            return Err(GenerateError::Uuid(format!(
                                "Invalid value for node_id: {}",
                                node_id
                            )))
                        }
                    }
                }
                None => return Err(GenerateError::Uuid("Missing node_id".to_string())),
            };

            Ok(Uuid::now_v1(node_id_slice).to_string())
        }
        _ => Err(GenerateError::Uuid(format!(
            "Version {} not implemented",
            version
        ))),
    }
}

#[cfg(test)]
mod tests {
    use uuid::{Uuid, Version};

    use super::*;

    #[test]
    fn test_token() {
        match token(10, true, true, true, true) {
            Ok(s) => panic!("{:#?}", s),
            Err(e) => match e {
                GenerateError::Token(s) => {
                    assert_eq!(s, "Nothing to generate");
                }
                _ => panic!("{:#?}", e),
            },
        }
        match token(10, false, true, true, true) {
            Ok(s) => assert_eq!(s.len(), 10),
            Err(e) => panic!("{:#?}", e),
        }

        match token(10, true, false, true, true) {
            Ok(s) => assert!(s.chars().all(|c| c.is_ascii_lowercase())),
            Err(e) => panic!("{:#?}", e),
        }

        match token(10, false, true, true, true) {
            Ok(s) => assert!(s.chars().all(|c| c.is_ascii_uppercase())),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_uuid_v3() {
        let u = Uuid::new_v3(&Uuid::NAMESPACE_DNS, &[]);
        assert_eq!(Some(Version::Md5), u.get_version());
    }

    #[test]
    fn test_uuid_v4() {
        let u = Uuid::new_v4();
        assert_eq!(Some(Version::Random), u.get_version());
    }

    #[test]
    fn test_uuid_v5() {
        let u = Uuid::new_v5(&Uuid::NAMESPACE_DNS, &[]);
        assert_eq!(Some(Version::Sha1), u.get_version());
    }
}
