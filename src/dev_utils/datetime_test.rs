use crate::dev_utils::datetime::*;

mod tests {
    use super::*;

    #[test]
    fn test_epoch_to_iso8601() {
        assert_eq!(
            convert("epoch", "iso8601", "1"),
            Ok("1970-01-01T00:00:01+00:00".to_string())
        );
    }
    #[test]
    fn test_unix_to_iso8601() {
        assert_eq!(
            convert("unix", "iso8601", "1"),
            Ok("1970-01-01T00:00:01+00:00".to_string())
        );
    }
    #[test]
    fn test_epoch_to_rfc3339() {
        assert_eq!(
            convert("epoch", "rfc3339", "1"),
            Ok("1970-01-01T00:00:01+00:00".to_string())
        );
    }
    #[test]
    fn test_unix_to_rfc3339() {
        assert_eq!(
            convert("unix", "rfc3339", "1"),
            Ok("1970-01-01T00:00:01+00:00".to_string())
        );
    }
    #[test]
    fn test_iso8601_to_epoch() {
        assert_eq!(
            convert("iso8601", "epoch", "1970-01-01T00:00:01+00:00"),
            Ok("1".to_string())
        );
    }
    #[test]
    fn test_iso8601_to_unix() {
        assert_eq!(
            convert("iso8601", "unix", "1970-01-01T00:00:01+00:00"),
            Ok("1".to_string())
        );
    }
    #[test]
    fn test_rfc3999_to_epoch() {
        assert_eq!(
            convert("rfc3339", "epoch", "1970-01-01T00:00:01+00:00"),
            Ok("1".to_string())
        );
    }
    #[test]
    fn test_rfc3339_to_unix() {
        assert_eq!(
            convert("rfc3339", "unix", "1970-01-01T00:00:01+00:00"),
            Ok("1".to_string())
        );
    }

    #[test]
    fn epoch_to_rfc2822() {
        assert_eq!(
            convert("epoch", "rfc2822", "1"),
            Ok("Thu, 1 Jan 1970 00:00:01 +0000".to_string())
        );
    }

    #[test]
    fn unix_to_rfc2822() {
        assert_eq!(
            convert("unix", "rfc2822", "1"),
            Ok("Thu, 1 Jan 1970 00:00:01 +0000".to_string())
        );
    }

    #[test]
    fn test_rfc2822_to_epoch() {
        assert_eq!(
            convert("rfc2822", "epoch", "Thu, 01 Jan 1970 00:00:01 GMT"),
            Ok("1".to_string())
        );
    }

    #[test]
    fn test_rfc2822_to_unix() {
        assert_eq!(
            convert("rfc2822", "unix", "Thu, 01 Jan 1970 00:00:01 GMT"),
            Ok("1".to_string())
        );
    }

    #[test]
    fn test_rfc2822_to_iso8601() {
        assert_eq!(
            convert("rfc2822", "iso8601", "Thu, 01 Jan 1970 00:00:01 GMT"),
            Ok("1970-01-01T00:00:01+00:00".to_string())
        );
    }

    #[test]
    fn test_rfc2822_to_rfc3339() {
        assert_eq!(
            convert("rfc2822", "rfc3339", "Thu, 01 Jan 1970 00:00:01 GMT"),
            Ok("1970-01-01T00:00:01+00:00".to_string())
        );
    }

    #[test]
    fn test_iso8601_to_rfc2822() {
        assert_eq!(
            convert("iso8601", "rfc2822", "1970-01-01T00:00:01+00:00"),
            Ok("Thu, 1 Jan 1970 00:00:01 +0000".to_string())
        )
    }

    #[test]
    fn test_rfc3339_to_rfc2822() {
        assert_eq!(
            convert("rfc3339", "rfc2822", "1970-01-01T00:00:01+00:00"),
            Ok("Thu, 1 Jan 1970 00:00:01 +0000".to_string())
        )
    }
}
