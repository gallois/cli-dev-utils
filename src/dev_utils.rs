pub mod convert;
pub mod hash;
pub mod url;

pub fn enum_variants<T: strum::VariantNames>() -> String {
    T::VARIANTS
        .iter()
        .map(|s| s.to_lowercase())
        .collect::<Vec<String>>()
        .join(", ")
}
