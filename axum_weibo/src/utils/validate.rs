use regex::Regex;
use validator::ValidationError;

pub fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    let r = Regex::new(r"^1[3456789]\d{9}$").unwrap();
    if r.is_match(phone) {
        return Ok(());
    }
    Err(ValidationError::new("10001"))
}