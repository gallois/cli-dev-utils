use crate::dev_utils::percentage::*;

mod tests {
    use super::*;

    #[test]
    fn test_to() {
        assert_eq!(to(100.0, 50.0, 0).unwrap(), "50%");
        assert_eq!(to(150.0, 50.0, 2).unwrap(), "33.33%");
    }

    #[test]
    fn test_of() {
        assert_eq!(of(50.0, 100.0, 0).unwrap(), "50");
        assert_eq!(of(33.33, 100.0, 2).unwrap(), "33.33");
        assert_eq!(of(25.0, 200.0, 3).unwrap(), "50.000");
    }

    #[test]
    fn test_change() {
        assert_eq!(change(100.0, 50.0, 0).unwrap(), "-50%");
        assert_eq!(change(50.0, 100.0, 0).unwrap(), "100%");
    }
}
