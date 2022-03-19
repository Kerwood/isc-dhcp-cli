use super::error::DhcpctlError;
use crate::{APP_NAME, CONFIG_NAME};
use colored::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfyConfig {
    pub api_url: String,
    pub auth_token: String,
}

impl Default for ConfyConfig {
    fn default() -> Self {
        ConfyConfig {
            api_url: "".to_string(),
            auth_token: "not-set".to_string(),
        }
    }
}

pub fn load_config() -> Result<ConfyConfig, DhcpctlError> {
    let conf: ConfyConfig = confy::load(APP_NAME, CONFIG_NAME)?;
    Ok(conf)
}

pub fn store_config(conf: &ConfyConfig) -> Result<(), DhcpctlError> {
    confy::store(APP_NAME, CONFIG_NAME, &conf)?;
    Ok(())
}

pub fn set_auth_token(token: &str) -> Result<(), DhcpctlError> {
    let mut conf: ConfyConfig = load_config()?;
    conf.auth_token = token.to_owned();
    store_config(&conf)?;
    Ok(())
}

pub fn get_auth_token() -> Result<String, DhcpctlError> {
    let conf: ConfyConfig = load_config()?;
    Ok(conf.auth_token)
}

pub fn set_api_url(url: &str) -> Result<(), DhcpctlError> {
    let mut conf: ConfyConfig = load_config()?;
    conf.api_url = url.to_owned();
    store_config(&conf)?;
    Ok(())
}

pub fn get_api_url() -> Result<String, DhcpctlError> {
    let conf: ConfyConfig = load_config()?;
    Ok(conf.api_url)
}

pub fn check_if_conf_exists() -> Result<(), DhcpctlError> {
    let file = confy::get_configuration_file_path(APP_NAME, CONFIG_NAME)?;
    if !Path::new(&file).exists() {
        println!("No config file found at {:?}, creating one for you.", &file);
        println!(
            "\nPlease set the API URL with '{}'",
            "dhcpctl set config --url https://ip-or-domain-name".green()
        );
        println!(
            "You can also add or remove headers on the API call, have a look at '{}' or '{}'",
            "dhcpctl set header".green(),
            "dhcpctl remove header".green()
        );
        let _conf: ConfyConfig = confy::load("dhcpctl", "config")?;
        return Err(DhcpctlError::MissingConfigFile);
    }

    Ok(())
}
