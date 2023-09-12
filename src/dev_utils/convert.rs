use csv;
use flatten_json_object::ArrayFormatting;
use flatten_json_object::Flattener;
use json_objects_to_csv::{Error, Json2Csv};
use std::str;
use std::str::Utf8Error;
use strum_macros::EnumString;
use strum_macros::{EnumIter, EnumVariantNames};

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum Conversion {
    Json2Csv,
    Json2Yaml,
    Csv2Tsv,
    String2Hex,
    Hex2String,
    Text2Nato,
}

#[derive(Debug)]
pub enum JsonYamlErrors {
    JsonError(serde_json::Error),
    YamlError(serde_yaml::Error),
}

#[derive(Debug)]
pub enum ConversionError {
    Json2Csv(Error),
    Json2Yaml(JsonYamlErrors),
    Utf8Error(Utf8Error),
    Hex2String(String),
}

pub fn json2csv(data: &str) -> Result<String, ConversionError> {
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

    match Json2Csv::new(flattener).convert_from_reader(data.as_bytes(), csv_writer) {
        Ok(_) => (),
        Err(e) => return Err(ConversionError::Json2Csv(e)),
    }

    let result = match str::from_utf8(&output) {
        Ok(s) => Ok(s.to_string()),
        Err(e) => return Err(ConversionError::Utf8Error(e)),
    };

    result
}

pub fn json2yaml(data: &str) -> Result<String, ConversionError> {
    let json_data: serde_yaml::Value = match serde_json::from_str(data) {
        Ok(v) => v,
        Err(e) => return Err(ConversionError::Json2Yaml(JsonYamlErrors::JsonError(e))),
    };

    let yaml_data = match serde_yaml::from_value::<serde_json::Value>(json_data) {
        Ok(v) => v,
        Err(e) => return Err(ConversionError::Json2Yaml(JsonYamlErrors::YamlError(e))),
    };
    let yaml_str = match serde_yaml::to_string(&yaml_data) {
        Ok(v) => v,
        Err(e) => return Err(ConversionError::Json2Yaml(JsonYamlErrors::YamlError(e))),
    };

    Ok(yaml_str)
}

pub fn csv2tsv(data: &str) -> String {
    let mut result: String = String::new();
    for line in data.lines() {
        result += &line.replace(',', "\t");
        result += "\n";
    }
    result.trim().to_string()
}

pub fn string2hex(data: &str) -> String {
    let mut result: String = String::new();
    for byte in data.bytes() {
        result += &format!("{:02x}", byte);
    }
    result.trim().to_string()
}

pub fn hex2string(data: &str) -> Result<String, ConversionError> {
    if data.len() % 2 != 0 {
        return Err(ConversionError::Hex2String(
            "Hex string must have an even number of characters".to_string(),
        ));
    }

    let mut result: String = String::new();
    let mut bytes = Vec::new();
    let mut hex_iter = data.chars();

    while let Some(hex1) = hex_iter.next() {
        let hex2 = match hex_iter.next() {
            Some(h) => h,
            None => {
                return Err(ConversionError::Hex2String(
                    "Invalid hex string".to_string(),
                ))
            }
        };

        let byte = match u8::from_str_radix(&format!("{}{}", hex1, hex2), 16) {
            Ok(b) => b,
            Err(_) => {
                return Err(ConversionError::Hex2String(format!(
                    "Invalid hex character {}{}",
                    hex1, hex2
                )))
            }
        };
        bytes.push(byte);
    }

    result.push_str(
        &String::from_utf8(bytes)
            .map_err(|_| ConversionError::Hex2String("Invalid hex string".to_string()))?,
    );

    Ok(result)
}

pub fn text2nato(content: &str) -> String {
    let mut result = String::new();
    for ch in content.chars() {
        let ch = match ch {
            'a' | 'A' => "Alpha".to_string(),
            'b' | 'B' => "Bravo".to_string(),
            'c' | 'C' => "Charlie".to_string(),
            'd' | 'D' => "Delta".to_string(),
            'e' | 'E' => "Echo".to_string(),
            'f' | 'F' => "Foxtrot".to_string(),
            'g' | 'G' => "Golf".to_string(),
            'h' | 'H' => "Hotel".to_string(),
            'i' | 'I' => "India".to_string(),
            'j' | 'J' => "Juliet".to_string(),
            'k' | 'K' => "Kilo".to_string(),
            'l' | 'L' => "Lima".to_string(),
            'm' | 'M' => "Mike".to_string(),
            'n' | 'N' => "November".to_string(),
            'o' | 'O' => "Oscar".to_string(),
            'p' | 'P' => "Papa".to_string(),
            'q' | 'Q' => "Quebec".to_string(),
            'r' | 'R' => "Romeo".to_string(),
            's' | 'S' => "Sierra".to_string(),
            't' | 'T' => "Tango".to_string(),
            'u' | 'U' => "Uniform".to_string(),
            'v' | 'V' => "Victor".to_string(),
            'w' | 'W' => "Whiskey".to_string(),
            'x' | 'X' => "X-ray".to_string(),
            'y' | 'Y' => "Yankee".to_string(),
            'z' | 'Z' => "Zulu".to_string(),
            _ => ch.to_string(),
        };
        result.push_str(&format!("{} ", &ch));
    }
    result.trim().to_string()
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

    #[test]
    fn test_json2yaml() {
        let result = json2yaml("{\"a\": 1, \"b\": 2, \"c\": 3}");
        match result {
            Ok(s) => assert_eq!(s, "a: 1\nb: 2\nc: 3\n"),
            Err(e) => panic!("{:#?}", e),
        }
        let data = r#"
        {
            "checked": false,
            "dimensions": {
                "width": 5,
                "height": 10
            },
            "id": 1,
            "name": "A green door",
            "price": 12.5,
            "tags": [
                "home",
                "green"
            ]
        }
        "#;
        let result = json2yaml(data);
        match result {
            Ok(s) => assert_eq!(s, "checked: false\ndimensions:\n  height: 10\n  width: 5\nid: 1\nname: A green door\nprice: 12.5\ntags:\n- home\n- green\n"),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_csv2tsv() {
        let result = csv2tsv("a,b,c\n1,2,3");
        assert_eq!(result, "a\tb\tc\n1\t2\t3");
    }

    #[test]
    fn test_string2hex() {
        let result = string2hex("abc");
        assert_eq!(result, "616263");
    }

    #[test]
    fn test_hex2string() {
        let result = hex2string("616263");
        match result {
            Ok(s) => assert_eq!(s, "abc"),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn test_text2nato() {
        let result = text2nato("abc");
        assert_eq!(result, "Alpha Bravo Charlie");
    }
}
