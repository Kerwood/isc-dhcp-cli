use super::config;
use super::config::ConfyConfig;
use reqwest;
use reqwest::header::AUTHORIZATION;
use serde::de::DeserializeOwned;
use std::error::Error;
use std::process::exit;

pub async fn run<T: DeserializeOwned>(path: &str) -> Result<T, Box<dyn Error>> {
    let config: ConfyConfig = config::load_config()?;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}{}", config.api_url, path))
        .header(AUTHORIZATION, config.auth_token)
        .send()
        .await?;

    match response.error_for_status() {
        Ok(res) => Ok(res.json::<T>().await?),
        Err(error) => {
            println!("{}", error);
            exit(2);
        }
    }
}
