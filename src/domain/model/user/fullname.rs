use std::str::FromStr;

use derive_getters::Getters;
use regex::Regex;

use crate::model::custom_error::ValidationError;

#[derive(Clone, Debug, Getters, PartialEq, Eq)]
pub struct FullName {
    first_name: Name,
    last_name: Name,
}

impl FullName {
    pub fn new(first_name: &String, last_name: &String) -> anyhow::Result<Self> {
        Ok(Self {
            first_name: first_name.parse()?,
            last_name: last_name.parse()?,
        })
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(String);

impl FromStr for Name {
    type Err = ValidationError::InvalidFullName;
    fn from_str(name: &String) -> Result<Self, Self::Err> {
        let trimmed_name = name.trim();
        if trimmed_name.is_empty() {
            ValidationError::InvalidFullName("At least one character.".to_string());
        }

        let regex = Regex::new(r#"^[a-zA-Z]+$"#).unwrap();
        if !regex.is_match(trimmed_name) {
            ValidationError::InvalidFullName("Non-alphabetic characters are not allowed.".to_string());
        }
        Ok(Name(trimmed_name.to_string()))
    }
}
