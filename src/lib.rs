use std::env;

pub fn fetch_environment_variable(name: &str) -> String {
    env::var(name).expect(&format!("{} is required", name))
}