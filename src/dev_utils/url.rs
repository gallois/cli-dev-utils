use strum_macros::{EnumIter, EnumVariantNames};
use urlencoding;

#[derive(EnumIter, EnumVariantNames)]
pub enum UrlAction {
    Encode,
}

pub fn encode(url: &str) -> String {
    urlencoding::encode(url).into_owned()
}
