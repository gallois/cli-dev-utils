use crate::dev_utils::url::*;

mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(
            encode("https://theworkoutcalculator.com/"),
            "https%3A%2F%2Ftheworkoutcalculator.com%2F"
        );
    }

    #[test]
    fn test_decode() {
        assert_eq!(
            decode("https%3A%2F%2Ftheworkoutcalculator.com%2F"),
            Ok("https://theworkoutcalculator.com/".to_string())
        );
    }

    #[test]
    fn test_parse() {
        let result =
            parse("https://foo:bar@theworkoutcalculator.com/path?param1=value1&param2=value2");
        match result {
            Ok(s) => assert_eq!(
                s,
                "https://foo:bar@theworkoutcalculator.com/path?param1=value1&param2=value2\nscheme: https\nusername: foo\npassword: bar\nhost: theworkoutcalculator.com\npath: /path\nparams\n\tparam1: value1\n\tparam2: value2"
            ),
            Err(e) => panic!("{}", e),
        }
    }
}
