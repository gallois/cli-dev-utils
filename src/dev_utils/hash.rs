use md5 as md5crate;
use sha2::{Digest, Sha256, Sha512};
use strum_macros::{EnumIter, EnumString, EnumVariantNames};

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum HashType {
    Md5,
    Sha256,
    Sha512,
}

pub fn md5(content: &str) -> String {
    format!("{:x}", md5crate::compute(content))
}

pub fn sha256(content: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.update(content.as_bytes());
    let result = sha256.finalize();
    format!("{:x}", result)
}

pub fn sha512(content: &str) -> String {
    let mut sha512 = Sha512::new();
    sha512.update(content.as_bytes());
    let result = sha512.finalize();
    format!("{:x}", result)
}

#[cfg(test)]
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
