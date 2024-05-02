use serde::{Deserialize, Serialize};
use crate::environment_variable::EnvironmentVariable;
use crate::vercel_environment_configuration::VercelEnvironmentConfiguration;

#[derive(Builder, Serialize, Deserialize)]
pub struct VercelEnvironmentVariable {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub variable_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "target")]
    pub target_environment: Vec<String>,
    #[serde(rename = "gitBranch", skip_serializing_if = "Option::is_none")]
    pub github_branch: Option<String>,
}

pub fn build_vercel_environment_variable(
    vercel_environment_configuration: VercelEnvironmentConfiguration,
    environment_variable: EnvironmentVariable,
) -> VercelEnvironmentVariable {
    VercelEnvironmentVariableBuilder::default()
        .key(environment_variable.key)
        .value(environment_variable.value)
        .variable_type(match environment_variable.variable_type {
            Ok(value) => value.to_string(),
            Err(e) => e,
        })
        .comment(Option::from(environment_variable.comment))
        .target_environment(match vercel_environment_configuration.target_environment {
            Ok(value) => vec![value.to_string()],
            Err(e) => vec![e],
        })
        .github_branch(vercel_environment_configuration.github_branch)
        .build()
        .unwrap()
}