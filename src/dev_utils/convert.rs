use csv;
use flatten_json_object::ArrayFormatting;
use flatten_json_object::Flattener;
use json_objects_to_csv::{Error, Json2Csv};
use std::str;
use std::str::Utf8Error;
use strum_macros::{EnumIter, EnumVariantNames};

#[derive(EnumIter, EnumVariantNames)]
pub enum Conversion {
    Json2Csv,
}

#[derive(Debug)]
pub enum ConversionError {
    Json2Csv(Error),
    Utf8Error(Utf8Error),
}

pub fn json2csv(_json: &str) -> Result<String, ConversionError> {
    let flattener = Flattener::new()
        .set_key_separator(".")
        .set_array_formatting(ArrayFormatting::Surrounded {
            start: "[".to_string(),
            end: "]".to_string(),
        })
        .set_preserve_empty_arrays(false)
        .set_preserve_empty_objects(false);

    let mut output = Vec::<u8>::new();
    let csv_writer = csv::WriterBuilder::new()
        .delimiter(b',')
        .from_writer(&mut output);

    match Json2Csv::new(flattener).convert_from_reader(_json.as_bytes(), csv_writer) {
        Ok(_) => (),
        Err(e) => return Err(ConversionError::Json2Csv(e)),
    }

    let result = match str::from_utf8(&output) {
        Ok(s) => Ok(s.to_string()),
        Err(e) => return Err(ConversionError::Utf8Error(e)),
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json2csv() {
        let result = json2csv("{\"a\": 1, \"b\": 2, \"c\": 3}");
        match result {
            Ok(s) => assert_eq!(s, "a,b,c\n1,2,3\n"),
            Err(e) => panic!("{:#?}", e),
        }
    }
}
