use std::{
    fmt::{Display, Formatter},
    string::FromUtf8Error,
};

use strum_macros::{EnumIter, EnumString, EnumVariantNames};
use url::{ParseError, Url};
use urlencoding;

use std::str;

#[derive(EnumIter, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum UrlAction {
    Encode,
    Decode,
    Parse,
}

struct ParsedUrl {
    url: String,
    scheme: String,
    username: Option<String>,
    password: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    path: Option<String>,
    params: Vec<(String, String)>,
}

impl Display for ParsedUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out: String = String::new();
        out.push_str(&self.url);
        out.push_str(&format!("\nscheme: {}", self.scheme));
        if let Some(username) = self.username.clone() {
            out.push_str(&format!("\nusername: {}", username));
        };
        if let Some(password) = self.password.clone() {
            out.push_str(&format!("\npassword: {}", password));
        };
        if let Some(host) = self.host.clone() {
            out.push_str(&format!("\nhost: {}", host));
        };
        if let Some(port) = self.port {
            out.push_str(&format!("\nport: {}", port));
        };
        if let Some(path) = self.path.clone() {
            out.push_str(&format!("\npath: {}", path));
        };
        if !self.params.is_empty() {
            out.push_str("\nparams")
        }
        for (key, value) in &self.params {
            out.push_str(&format!("\n\t{}: {}", key, value));
        }

        write!(f, "{}", out)
    }
}

pub fn encode(url: &str) -> String {
    urlencoding::encode(url).into_owned()
}

pub fn decode(url: &str) -> Result<String, FromUtf8Error> {
    match urlencoding::decode(url) {
        Ok(decoded) => Ok(decoded.into_owned()),
        Err(e) => Err(e),
    }
}

pub fn parse(url: &str) -> Result<String, ParseError> {
    let parsed = Url::parse(url)?;

    let parsed_struct = ParsedUrl {
        url: parsed.to_string(),
        scheme: parsed.scheme().to_string(),
        username: Some(parsed.username().to_string()),
        password: parsed.password().map(|p| p.to_string()),
        host: parsed.host_str().map(|h| h.to_string()),
        port: parsed.port(),
        path: Some(parsed.path().to_string()),
        params: parsed
            .query_pairs()
            .map(|(p, v)| (p.to_string(), v.to_string()))
            .collect::<Vec<_>>(),
    };

    Ok(parsed_struct.to_string())
}

#[cfg(test)]
#[path = "./url_test.rs"]
mod url_test;
