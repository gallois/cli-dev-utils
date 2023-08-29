use strum_macros::{EnumIter, EnumVariantNames};

#[derive(EnumIter, EnumVariantNames)]
pub enum Conversion {
    Json2Csv,
}

pub fn json2csv(_json: &str) -> String {
    unimplemented!()
}
