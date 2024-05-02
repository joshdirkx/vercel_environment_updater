use serde::{Deserialize, Serialize};
use vercel_environment_updater::fetch_environment_variable;
use crate::variable_type::VariableType;

#[derive(Builder, Serialize, Deserialize)]
pub struct EnvironmentVariable {
    pub key: String,
    pub value: String,
    pub variable_type: Result<VariableType, String>,
    pub comment: Option<String>,
}

pub fn build_environment_variable() -> EnvironmentVariable {
    EnvironmentVariableBuilder::default()
        .key(fetch_environment_variable("KEY"))
        .value(fetch_environment_variable("VALUE"))
        .variable_type(VariableType::from_string(fetch_environment_variable(
            "VARIABLE_TYPE"
        )))
        .comment(Option::from(fetch_environment_variable("COMMENT")))
        .build()
        .unwrap()
}
