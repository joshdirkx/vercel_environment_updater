mod target_environment;
mod variable_type;
mod vercel_configuration;
mod environment_variable;
mod vercel_environment_configuration;
mod vercel_environment_variable;

#[macro_use]
extern crate derive_builder;

use reqwest::{Client, Error};
use vercel_configuration::build_vercel_configuration;
use vercel_environment_configuration::build_vercel_environment_configuration;
use environment_variable::build_environment_variable;
use vercel_environment_variable::build_vercel_environment_variable;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let vercel_configuration = build_vercel_configuration();
    let vercel_environment_configuration = build_vercel_environment_configuration();
    let environment_variable = build_environment_variable();
    let vercel_environment_variable = build_vercel_environment_variable(vercel_environment_configuration, environment_variable);

    let http_client = Client::new();

    let res = http_client
        .post(vercel_api_path(vercel_configuration.team_id, vercel_configuration.project_id))
        .bearer_auth(vercel_configuration.token)
        .json(&vercel_environment_variable)
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

fn base_vercel_path() -> String {
    "https://api.vercel.com/v10".to_string()
}