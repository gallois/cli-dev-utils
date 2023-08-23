use md5 as md5crate;
use sha2::{Digest, Sha256};

pub fn md5(content: &str) -> String {
    format!("{:x}", md5crate::compute(content))
}

pub fn sha256(content: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.update(content.as_bytes());
    let result = sha256.finalize();
    format!("{:x}", result)
}
