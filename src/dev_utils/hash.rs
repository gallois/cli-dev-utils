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
#[path = "./hash_test.rs"]
mod hash_test;
