use strum_macros::{EnumIter, EnumString, EnumVariantNames};
use capitalize;
use capitalize::Capitalize;

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum ListAction {
    Sort,
    Lowercase,
    Uppercase,
    Capitalise,
    Capitalize,
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
}
