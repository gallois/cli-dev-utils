use clap_stdin::MaybeStdin;

pub mod base64;
pub mod convert;
pub mod datetime;
pub mod date;
pub mod hash;
pub mod list;
pub mod url;
pub mod colour;


pub enum CliError {
    NoDataProvided,
    EditorError,
}

pub fn enum_variants<T: strum::VariantNames>() -> String {
    T::VARIANTS
        .iter()
        .map(|s| s.to_lowercase())
        .collect::<Vec<String>>()
        .join(", ")
}

pub fn get_content(
    content: Option<MaybeStdin<String>>,
    use_editor: bool,
) -> Result<String, CliError> {
    if !use_editor && content.is_none() {
        return Err(CliError::NoDataProvided);
    }

    let mut result = String::new();
    if let Some(c) = content {
        result = c.as_str().to_string();
    }

    if use_editor {
        result = match edit::edit(result) {
            Ok(content) => content.trim().to_string(),
            Err(_) => {
                return Err(CliError::EditorError);
            }
        }
    }

    Ok(result)
}
