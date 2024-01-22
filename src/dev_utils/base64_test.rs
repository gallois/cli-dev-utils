use crate::dev_utils::base64::*;

mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(
            encode("https://theworkoutcalculator.com/"),
            "aHR0cHM6Ly90aGV3b3Jrb3V0Y2FsY3VsYXRvci5jb20v"
        );
    }

    #[test]
    fn test_decode() {
        assert_eq!(
            decode("aHR0cHM6Ly90aGV3b3Jrb3V0Y2FsY3VsYXRvci5jb20v"),
            Ok("https://theworkoutcalculator.com/".to_string())
        )
    }
}
