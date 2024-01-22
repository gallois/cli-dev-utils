use crate::dev_utils::colour::*;

mod tests {
    use super::*;

    #[test]
    fn test_hex2rgb() {
        let result = hex2rgb("#1EA54C");
        match result {
            Ok(s) => assert_eq!(s, "rgb(30,165,76)"),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_hex2hsl() {
        let result = hex2hsl("#1EA54C");
        match result {
            Ok(s) => assert_eq!(s, "hsl(140,69%,38%)"),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_rgb2hex() {
        let result = rgb2hex("rgb(30,165,76)");
        match result {
            Ok(s) => assert_eq!(s, "#1ea54c"),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_hsl2hex() {
        let result = hsl2hex("hsl(140,69%,38%)");
        match result {
            Ok(s) => assert_eq!(s, "#1ea44b"),
            Err(e) => panic!("{:#?}", e),
        }
    }
}
