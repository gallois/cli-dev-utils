use capitalize;
use capitalize::Capitalize;
use rand::seq::SliceRandom;
use strum_macros::{EnumIter, EnumString, EnumVariantNames};
// TODO https://github.com/chrislearn/cruet might be more flexible
use itertools::Itertools;

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum ListAction {
    Sort,
    Lowercase,
    Uppercase,
    Capitalise,
    Capitalize,
    Reverse,
    Deduplicate,
    Unique,
    Dedup,
    Shuffle,
    Slice,
    Count,
}

pub fn sort(content: &str, separator: &str) -> String {
    let mut tokens = content.split(separator).collect::<Vec<&str>>();
    tokens.sort();

    tokens.join(separator)
}

pub fn lowercase(content: &str, separator: &str) -> String {
    let tokens = content
        .split(separator)
        .map(|t| t.to_lowercase())
        .collect::<Vec<String>>();

    tokens.join(separator)
}

pub fn uppercase(content: &str, separator: &str) -> String {
    let tokens = content
        .split(separator)
        .map(|t| t.to_uppercase())
        .collect::<Vec<String>>();

    tokens.join(separator)
}

pub fn capitalise(content: &str, separator: &str) -> String {
    let tokens = content
        .split(separator)
        .map(|t| t.capitalize())
        .collect::<Vec<String>>();

    tokens.join(separator)
}

pub fn reverse(content: &str, separator: &str) -> String {
    let mut tokens = content
        .split(separator)
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    tokens.reverse();

    tokens.join(separator)
}

pub fn deduplicate(content: &str, separator: &str) -> String {
    let tokens = content
        .split(separator)
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    tokens.iter().unique().join(separator)
}

pub fn shuffle(content: &str, separator: &str) -> String {
    let mut tokens = content
        .split(separator)
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    tokens.shuffle(&mut rand::thread_rng());
    tokens.join(separator)
}

pub fn slice(content: &str, separator: &str, index: usize, length: usize) -> String {
    let tokens = content
        .split(separator)
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    tokens
        .iter()
        .skip(index)
        .take(length)
        .cloned()
        .collect::<Vec<String>>()
        .join(separator)
}

pub fn count(content: &str, separator: &str) -> usize {
    if content.is_empty() {
        return 0;
    }
    content.split(separator).count()
}

#[cfg(test)]
#[path = "./list_test.rs"]
mod list_test;
