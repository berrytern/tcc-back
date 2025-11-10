
use {
    once_cell::sync::Lazy,
    regex::Regex, // no backtrack
    fancy_regex::Regex as Regex2, // backtrack
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
pub static RE_EMAIL: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap());
pub static RE_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Za-z\s]+$").unwrap());
pub static RE_PASSWORD: Lazy<Regex2> = Lazy::new(|| Regex2::new(r"^((?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[^A-Za-z\d]).{8,})$").unwrap());


#[derive(Debug,Serialize,Clone,ToSchema)]
pub struct Email(pub String);
impl<'de> Deserialize<'de> for Email  {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if RE_EMAIL.is_match(&s) {
            Ok(Email(s))
        } else {
            Err(serde::de::Error::custom("Invalid email format"))
        }
    }
}
impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}
impl From<String> for Email  {
    fn from(s: String) -> Self {
        Email(s)
    }
}

#[derive(Debug,Serialize,Clone,ToSchema)]
pub struct Name(pub String);
impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if RE_NAME.is_match(&s) {
            Ok(Name(s))
        } else {
            Err(serde::de::Error::custom("Invalid name format"))
        }
    }
}
impl From<Name> for String {
    fn from(name: Name) -> Self {
        name.0
    }
}
impl From<String> for Name {
    fn from(s: String) -> Self {
        Name(s)
    }
}

#[derive(Debug,Serialize,Clone,ToSchema)]
pub struct Password(pub String);
impl<'de> Deserialize<'de> for Password  {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match RE_PASSWORD.is_match(&s) {
            Ok(true) => Ok(Password(s)),
            _ => Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str("invalid password"), &"Password must be at least 8 characters long, contain at least one uppercase letter, one lowercase letter, one digit, and one special character.")),
        }
    }
}