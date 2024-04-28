use std::env;
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
    // #[serde(rename = "gitBranch")]
    // github_branch_name: String,
    comment: String,
}

// #[derive(Serialize, Deserialize)]
// enum VariableType {
//     System,
//     Secret,
//     Encrypted,
//     Plain,
//     Sensitive
// }
//
// impl VariableType {
//     fn from_string(variable_type_name: String) -> Result<Self, String> {
//         match variable_type_name.to_lowercase().as_str() {
//             "system" => Ok(VariableType::System),
//             "secret" => Ok(VariableType::Secret),
//             "encrypted" => Ok(VariableType::Encrypted),
//             "plain" => Ok(VariableType::Plain),
//             "sensitive" => Ok(VariableType::Sensitive),
//             _ => Err(format!("Invalid type for VariableType: {}", variable_type_name)),
//         }
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Error> {
    // fetch vercel requirements
    let vercel_token = fetch_environment_variable("VERCEL_TOKEN");
    let vercel_team_id = fetch_environment_variable("VERCEL_TEAM_ID");
    let vercel_project_id = fetch_environment_variable("VERCEL_PROJECT_ID");

    // fetch environment variable requirements
    let key = fetch_environment_variable("KEY");
    let value = fetch_environment_variable("VALUE");
    let target_environment = fetch_environment_variable("TARGET_ENVIRONMENT");
    let variable_type = fetch_environment_variable("VARIABLE_TYPE");
    // GitHub branch name is only supported for non-production environments it seems
    // let environment_variable_github_branch_name = "main"; //fetch_environment_variable("ENVIRONMENT_VARIABLE_GITHUB_BRANCH_NAME");
    let comment = fetch_environment_variable("COMMENT");

    // let variable_type = VariableType::from_string(environment_variable_variable_type.to_string())
    //     .expect("Invalid variable_type");

    let http_client = Client::new();

    let environment_variable = EnvironmentVariable {
        key: key.to_string(),
        value: value.to_string(),
        target_environment: vec![target_environment.to_string()],
        variable_type: variable_type.to_string(),
        // github_branch_name: environment_variable_github_branch_name.to_string(),
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