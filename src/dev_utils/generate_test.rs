use crate::dev_utils::generate::*;

mod tests {
    use uuid::{Uuid, Version};

    use super::*;

    #[test]
    fn test_token() {
        match token(10, true, true, true, true) {
            Ok(s) => panic!("{:#?}", s),
            Err(e) => match e {
                GenerateError::Token(s) => {
                    assert_eq!(s, "Nothing to generate");
                }
                _ => panic!("{:#?}", e),
            },
        }
        match token(10, false, true, true, true) {
            Ok(s) => assert_eq!(s.len(), 10),
            Err(e) => panic!("{:#?}", e),
        }

        match token(10, true, false, true, true) {
            Ok(s) => assert!(s.chars().all(|c| c.is_ascii_lowercase())),
            Err(e) => panic!("{:#?}", e),
        }

        match token(10, false, true, true, true) {
            Ok(s) => assert!(s.chars().all(|c| c.is_ascii_uppercase())),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_uuid_v3() {
        let u = Uuid::new_v3(&Uuid::NAMESPACE_DNS, &[]);
        assert_eq!(Some(Version::Md5), u.get_version());
    }

    #[test]
    fn test_uuid_v4() {
        let u = Uuid::new_v4();
        assert_eq!(Some(Version::Random), u.get_version());
    }

    #[test]
    fn test_uuid_v5() {
        let u = Uuid::new_v5(&Uuid::NAMESPACE_DNS, &[]);
        assert_eq!(Some(Version::Sha1), u.get_version());
    }
}
