use crate::config::validators::{REGEX_EMBED_COLOR, validate_ipv4};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct Config {
    #[validate(custom(function = "validate_ipv4"))]
    pub host: String,
    #[validate(range(min = 1, max = u16::MAX))]
    pub port: u16,
    #[validate(url)]
    pub hostname: String,
    #[validate(length(min = 1, max = 30))]
    pub oembed_endpoint: String,
    pub provider_name: String,
    #[validate(url)]
    pub provider_url: String,
    #[validate(regex(path = *REGEX_EMBED_COLOR))]
    pub embed_color: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            host: "127.0.0.1".to_string(),
            port: 8000,
            hostname: "http://127.0.0.1:8000".to_string(),
            oembed_endpoint: "oembed".to_string(),
            provider_name: "9GAG".to_string(),
            provider_url: "https://9gag.com".to_string(),
            embed_color: "#FFFFFF".to_string(),
        }
    }
}
