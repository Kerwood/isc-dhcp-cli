use super::error::DhcpctlError;
use super::reqwest_handler;
use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Globals {
    pub authoritative: Option<bool>,
    #[serde(rename = "default-lease-time")]
    pub default_lease_time: Option<String>,
    #[serde(rename = "max-lease-time")]
    pub max_lease_time: Option<String>,
    pub options: Option<Options>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
    #[serde(rename = "domain-name")]
    pub domain_name: Option<String>,
    #[serde(rename = "domain-name-servers")]
    pub domain_name_servers: Option<Vec<String>>,
}

pub async fn list_globals() -> Result<(), DhcpctlError> {
    let payload: Globals = reqwest_handler::run("/config/globals").await?;

    println!("");
    if let Some(v) = payload.authoritative {
        println!("{} {}", "Authoritative: ".bold(), v);
    }
    if let Some(v) = &payload.default_lease_time {
        println!("{} {}", "Default Lease Time: ".bold(), v);
    }
    if let Some(v) = &payload.max_lease_time {
        println!("{} {}", "Max Lease Time: ".bold(), v);
    }
    if let Some(v) = payload.options {
        println!("");
        if let Some(x) = v.domain_name {
            println!("{} {}", "Domain name: ".bold(), x);
        }
        if let Some(x) = v.domain_name_servers {
            println!("{} {}", "DNS Servers: ".bold(), x.join(", "));
        }
    }
    Ok(())
}
