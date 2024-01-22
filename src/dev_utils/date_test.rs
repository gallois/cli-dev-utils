use crate::dev_utils::date::*;

mod tests {
    use super::*;

    #[test]
    fn test_delta() {
        let result = delta("1d", 0);
        match result {
            Ok(s) => assert_eq!(s, "1970-01-02"),
            Err(e) => panic!("{:#?}", e),
        }

        let result = delta("1m", 0);
        match result {
            Ok(s) => assert_eq!(s, "1970-02-01"),
            Err(e) => panic!("{:#?}", e),
        }

        let result = delta("-1d", 88000);
        match result {
            Ok(s) => assert_eq!(s, "1970-01-01"),
            Err(e) => panic!("{:#?}", e),
        }
    }
}
