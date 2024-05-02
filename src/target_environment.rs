use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum TargetEnvironment {
    Production,
    Preview,
    Development,
}

impl TargetEnvironment {
    pub fn from_string(name: String) -> Result<Self, String> {
        match name.to_lowercase().as_str() {
            "production" => Ok(TargetEnvironment::Production),
            "preview" => Ok(TargetEnvironment::Preview),
            "development" => Ok(TargetEnvironment::Development),
            _ => Err(format!("Invalid name for TargetEnvironment: {}", name)),
        }
    }
}

impl fmt::Display for TargetEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            TargetEnvironment::Production => "production",
            TargetEnvironment::Preview => "preview",
            TargetEnvironment::Development => "development",
        })
    }
}