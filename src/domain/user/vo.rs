use super::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);
impl Email {
    pub fn parse(raw: &str) -> Result<Self, DomainError> {
        if raw.contains('@') {
            Ok(Self(raw.to_owned()))
        } else {
            Err(DomainError::InvalidEmail)
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);
impl Username {
    pub fn parse(raw: &str) -> Result<Self, DomainError> {
        let ok_len = (3..=16).contains(&raw.len());
        let ok_char = raw.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
        if ok_len && ok_char {
            Ok(Self(raw.to_owned()))
        } else {
            Err(DomainError::InvalidUsername)
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordHash(String);
impl PasswordHash {
    pub fn from_hash(hash: String) -> Result<Self, DomainError> {
        let looks_supported = hash.starts_with("$2") || hash.starts_with("$argon2");
        if looks_supported {
            Ok(Self(hash))
        } else {
            Err(DomainError::UnsupportedHash)
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
