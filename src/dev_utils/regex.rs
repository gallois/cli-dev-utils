use strum_macros::{EnumIter, EnumString, EnumVariantNames};

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum RegexAction {
    Email,
}

pub fn email() -> String {
    r"^[\w\-\.]+@([\w-]+\.)+[\w-]{2,}$".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email() {
        assert_eq!(email(), r"^[\w\-\.]+@([\w-]+\.)+[\w-]{2,}$")
    }
}
