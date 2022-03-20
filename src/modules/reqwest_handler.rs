use super::config;
use super::config::ConfyConfig;
use super::error::DhcpctlError;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::de::DeserializeOwned;

pub async fn run<T: DeserializeOwned>(path: &str) -> Result<T, DhcpctlError> {
    let config: ConfyConfig = config::load_config()?;

    if config.api_url.is_empty() {
        return Err(DhcpctlError::MissingUrl);
    }

    let mut headers = HeaderMap::new();

    if !config.auth_token.is_empty() {
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&config.auth_token)?);
    }

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}{}", config.api_url, path))
        .headers(headers)
        .send()
        .await?;

    match response.error_for_status() {
        Ok(res) => Ok(res.json::<T>().await?),
        Err(error) => Err(DhcpctlError::BadStatusCode(error.to_string())),
    }
}
