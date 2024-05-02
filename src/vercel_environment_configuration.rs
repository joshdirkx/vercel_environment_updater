use serde::{Deserialize, Serialize};
use vercel_environment_updater::fetch_environment_variable;
use crate::target_environment::TargetEnvironment;

#[derive(Builder, Serialize, Deserialize)]
pub struct VercelEnvironmentConfiguration {
    #[serde(rename = "target")]
    pub target_environment: Result<TargetEnvironment, String>,
    #[serde(rename = "gitBranch", skip_serializing_if = "Option::is_none")]
    pub github_branch: Option<String>,
}

pub fn build_vercel_environment_configuration() -> VercelEnvironmentConfiguration {
    VercelEnvironmentConfigurationBuilder::default()
        .target_environment(TargetEnvironment::from_string(fetch_environment_variable("TARGET_ENVIRONMENT")))
        .github_branch(None)//Option::from(fetch_environment_variable("GITHUB_BRANCH")))
        .build()
        .unwrap()
}