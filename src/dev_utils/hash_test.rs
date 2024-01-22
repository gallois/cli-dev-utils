use crate::dev_utils::hash::*;

mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        assert_eq!(md5("foo"), "acbd18db4cc2f85cedef654fccc4a4d8");
    }

    #[test]
    fn test_sha256() {
        assert_eq!(
            sha256("foo"),
            "2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae"
        );
    }

    #[test]
    fn test_sha512() {
        assert_eq!(
            sha512("foo"),
            "f7fbba6e0636f890e56fbbf3283e524c6fa3204ae298382d624741d0dc6638326e282c41be5e4254d8820772c5518a2c5a8c0c7f7eda19594a7eb539453e1ed7"
        );
    }
}
