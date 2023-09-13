use strum_macros::{EnumIter, EnumString, EnumVariantNames};

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum ListAction {
    Sort,
}

pub fn sort(content: &str, separator: &str) -> String {
    let mut tokens = content.split(separator).collect::<Vec<&str>>();
    tokens.sort();

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
}
