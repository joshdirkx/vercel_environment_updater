use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum VariableType {
    System,
    Secret,
    Encrypted,
    Plain,
    Sensitive
}

impl VariableType {
    pub fn from_string(variable_type_name: String) -> Result<Self, String> {
        match variable_type_name.to_lowercase().as_str() {
            "system" => Ok(VariableType::System),
            "secret" => Ok(VariableType::Secret),
            "encrypted" => Ok(VariableType::Encrypted),
            "plain" => Ok(VariableType::Plain),
            "sensitive" => Ok(VariableType::Sensitive),
            _ => Err(format!("Invalid type for VariableType: {}", variable_type_name)),
        }
    }
}

impl fmt::Display for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            VariableType::System => "system",
            VariableType::Secret => "secret",
            VariableType::Encrypted => "encrypted",
            VariableType::Plain => "plain",
            VariableType::Sensitive => "sensitive",
        })
    }
}