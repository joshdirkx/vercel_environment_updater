use std::{env, fmt};
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct EnvironmentVariable {
    key: String,
    value: String,
    #[serde(rename = "target")]
    target_environment: Vec<String>,
    #[serde(rename = "type")]
    variable_type: String,
    // #[serde(rename = "gitBranch", skip_serializing_if = "Option::is_none")]
    // github_branch: Some<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    comment: String,
}

#[derive(Serialize, Deserialize)]
enum TargetEnvironment {
    Production,
    Preview,
    Development,
}

impl TargetEnvironment {
    fn from_string(name: String) -> Result<Self, String> {
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

#[derive(Serialize, Deserialize)]
enum VariableType {
    System,
    Secret,
    Encrypted,
    Plain,
    Sensitive
}

impl VariableType {
    fn from_string(variable_type_name: String) -> Result<Self, String> {
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

struct VercelConfiguration {
    token: String,
    team_id: String,
    project_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // fetch vercel requirements
    let vercel_token = fetch_environment_variable("VERCEL_TOKEN");
    let vercel_team_id = fetch_environment_variable("VERCEL_TEAM_ID");
    let vercel_project_id = fetch_environment_variable("VERCEL_PROJECT_ID");

    // fetch environment variable requirements
    let key = fetch_environment_variable("KEY");
    let value = fetch_environment_variable("VALUE");
    let target_environment = TargetEnvironment::from_string(fetch_environment_variable("TARGET_ENVIRONMENT"));
    let variable_type = VariableType::from_string(fetch_environment_variable("VARIABLE_TYPE"));
    // GitHub branch name is only supported for non-production environments it seems
    // let github_branch = fetch_environment_variable("GITHUB_BRANCH");
    let comment = fetch_environment_variable("COMMENT");

    let http_client = Client::new();

    let environment = match target_environment {
        Ok(value) => value.to_string(),
        Err(e) => e,
    };

    let ttype = match variable_type {
        Ok(value) => value.to_string(),
        Err(e) => e,
    };

    let environment_variable = EnvironmentVariable {
        key: key.to_string(),
        value: value.to_string(),
        target_environment: vec![environment],
        variable_type: ttype,
        // github_branch_name: github_branch.to_string(),
        comment: comment.to_string(),
    };

    let res = http_client
        .post(vercel_api_path(vercel_team_id, vercel_project_id))
        .bearer_auth(vercel_token)
        .json(&environment_variable)
        .send()
        .await?;

    println!("Url: {}", res.url());
    println!("Status: {}", res.status());
    println!("Body: {:?}", res.text().await?);

    Ok(())
}

fn vercel_api_path(vercel_team_id: String, vercel_project_id: String) -> String {
    format!("{}/projects/{}/env?teamId={}&upsert=true", base_vercel_path(), vercel_project_id, vercel_team_id)
}

fn fetch_environment_variable(name: &str) -> String {
    env::var(name).expect(&format!("{} is required", name))
}

fn base_vercel_path() -> String {
    "https://api.vercel.com/v10".to_string()
}