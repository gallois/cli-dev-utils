use std::fmt::{Display, Formatter};

use clap::{arg, Subcommand};
use rand::Rng;
use strum_macros::{EnumIter, EnumString, EnumVariantNames};

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
}

#[derive(Debug)]
pub enum GenerateError {
    TokenGenerate(String),
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
        return Err(GenerateError::TokenGenerate(
            "Nothing to generate".to_string(),
        ));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token() {
        match token(10, false, false, false, false) {
            Ok(s) => panic!("{:#?}", s),
            Err(e) => match e {
                GenerateError::TokenGenerate(s) => {
                    assert_eq!(s, "Nothing to generate");
                }
            },
        }
        match token(10, true, false, false, false) {
            Ok(s) => assert_eq!(s.len(), 10),
            Err(e) => panic!("{:#?}", e),
        }

        match token(10, false, true, false, false) {
            Ok(s) => assert!(s.chars().all(|c| c.is_ascii_lowercase())),
            Err(e) => panic!("{:#?}", e),
        }

        match token(10, true, false, false, false) {
            Ok(s) => assert!(s.chars().all(|c| c.is_ascii_uppercase())),
            Err(e) => panic!("{:#?}", e),
        }
    }
}
