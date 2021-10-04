use crate::{APP_NAME, CONFIG_NAME};
use colored::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;
use std::process;

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

pub fn load_config() -> Result<ConfyConfig, Box<dyn Error>> {
    let conf: ConfyConfig = confy::load(APP_NAME, CONFIG_NAME)?;
    Ok(conf)
}

pub fn store_config(conf: &ConfyConfig) -> Result<(), Box<dyn Error>> {
    confy::store(APP_NAME, CONFIG_NAME, &conf)?;
    Ok(())
}

pub fn set_auth_token(token: &str) -> Result<(), Box<dyn Error>> {
    let mut conf: ConfyConfig = load_config()?;
    conf.auth_token = token.to_owned();
    store_config(&conf)?;
    Ok(())
}

pub fn get_auth_token() -> Result<String, Box<dyn Error>> {
    let conf: ConfyConfig = load_config()?;
    Ok(conf.auth_token)
}

pub fn set_api_url(url: &str) -> Result<(), Box<dyn Error>> {
    let mut conf: ConfyConfig = load_config()?;
    conf.api_url = url.to_owned();
    store_config(&conf)?;
    Ok(())
}

pub fn get_api_url() -> Result<String, Box<dyn Error>> {
    let conf: ConfyConfig = load_config()?;
    Ok(conf.api_url)
}

pub fn check_if_conf_exists() {
    let file = confy::get_configuration_file_path(APP_NAME, CONFIG_NAME).unwrap();
    if !Path::new(&file).exists() {
        println!("No config file found at {:?}, creating one for you.", &file);
        println!(
            "\nPlease set the API URL with '{}'",
            "dhcpctl set url https://ip-or-domain-name".green()
        );
        println!(
            "You can also add or remove headers on the API call, have a look at '{}' or '{}'",
            "dhcpctl set header".green(),
            "dhcpctl remove header".green()
        );
        let _conf: ConfyConfig = confy::load("dhcpctl", "config").unwrap();
        process::exit(1)
    }
}
