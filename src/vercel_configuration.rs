use serde::{Deserialize, Serialize};
use vercel_environment_updater::fetch_environment_variable;

#[derive(Builder, Serialize, Deserialize, Debug, Clone)]
pub struct VercelConfiguration {
    pub token: String,
    pub team_id: String,
    pub project_id: String,
}

pub fn build_vercel_configuration() -> VercelConfiguration {
    VercelConfigurationBuilder::default()
        .token(fetch_environment_variable("VERCEL_TOKEN"))
        .team_id(fetch_environment_variable("VERCEL_TEAM_ID"))
        .project_id(fetch_environment_variable("VERCEL_PROJECT_ID"))
        .build()
        .unwrap()
}