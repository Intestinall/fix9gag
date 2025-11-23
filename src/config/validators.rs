use regex::Regex;
use std::net::Ipv4Addr;
use std::sync::LazyLock;
use validator::{ValidateIp, ValidationError};

pub static REGEX_EMBED_COLOR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"#([0-9a-fA-F]{3}){1,2}").unwrap());

pub fn validate_ipv4(ip: &str) -> Result<(), ValidationError> {
    match ip.parse::<Ipv4Addr>() {
        Ok(ip) => {
            if ip.validate_ipv4() {
                Ok(())
            } else {
                Err(ValidationError::new("Invalid IPv4 for host"))
            }
        }
        Err(_) => Err(ValidationError::new("Invalid IPv4 for host")),
    }
}
