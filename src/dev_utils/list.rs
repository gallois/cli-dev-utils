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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        assert_eq!(
            sort("one,two,three,four,five", ","),
            "five,four,one,three,two"
        );

        assert_eq!(
            sort("one two three four five", " "),
            "five four one three two"
        );
    }

    #[test]
    fn test_lowercase() {
        assert_eq!(
            lowercase("One Two Three Four FIVE", " "),
            "one two three four five"
        );
        assert_eq!(
            lowercase("One,Two,Three,Four,FIVE", ","),
            "one,two,three,four,five"
        );
    }

    #[test]
    fn test_uppercase() {
        assert_eq!(
            uppercase("One Two Three Four FIVE", " "),
            "ONE TWO THREE FOUR FIVE"
        );
        assert_eq!(
            uppercase("One,Two,Three,Four,FIVE", ","),
            "ONE,TWO,THREE,FOUR,FIVE"
        );
    }

    #[test]
    fn test_capitalise() {
        assert_eq!(
            capitalise("One Two Three Four FIVE", " "),
            "One Two Three Four Five"
        );
        assert_eq!(
            capitalise("One,Two,Three,Four,FIVE", ","),
            "One,Two,Three,Four,Five"
        );
    }

    #[test]
    fn test_reverse() {
        assert_eq!(
            reverse("One Two Three Four Five", " "),
            "Five Four Three Two One"
        )
    }

    #[test]
    fn test_deduplicate() {
        assert_eq!(
            deduplicate("One Two Three Four Five Five", " "),
            "One Two Three Four Five"
        );
        assert_eq!(
            deduplicate("One Two Three Four Five Five Four", " "),
            "One Two Three Four Five"
        );
    }

    #[test]
    fn test_slice() {
        assert_eq!(
            slice("One Two Three Four Five Five Four", " ", 0, 3),
            "One Two Three"
        );
        assert_eq!(
            slice("One Two Three Four Five Five Four", " ", 1, 3),
            "Two Three Four"
        );
        assert_eq!(slice("One Two Three Four Five Five Four", " ", 1, 1), "Two");
        assert_eq!(slice("One Two Three Four Five Five Four", " ", 8, 1), "");
    }
}
