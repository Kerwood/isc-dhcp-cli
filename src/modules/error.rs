use chrono::format::ParseError;
use confy::ConfyError;
use reqwest::Error as ReqwestError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DhcpctlError {
    MissingConfigFile,
    MissingUrl,
    NotValidCIDR,
    Reqwest(ReqwestError),
    Confy(ConfyError),
    ParseError(ParseError),
}

impl Error for DhcpctlError {}

impl fmt::Display for DhcpctlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DhcpctlError::MissingConfigFile => write!(f, "Config file not found."),
            DhcpctlError::MissingUrl => write!(f, "The URL for the ISC DHCP API is missing. Set it with 'dhcpctl set config --url https://ip-or-domain-name'"),
            DhcpctlError::NotValidCIDR => write!(f, "Not a valid CIDR."),
            DhcpctlError::Reqwest(e) => write!(f, "{}", e.to_string()),
            DhcpctlError::Confy(e) => write!(f, "[config-file] {}", e.to_string()),
            DhcpctlError::ParseError(e) => write!(f, "{}", e.to_string()),
        }
    }
}

impl From<reqwest::Error> for DhcpctlError {
    fn from(error: reqwest::Error) -> Self {
        DhcpctlError::Reqwest(error)
    }
}

impl From<ConfyError> for DhcpctlError {
    fn from(error: ConfyError) -> Self {
        DhcpctlError::Confy(error)
    }
}

impl From<ParseError> for DhcpctlError {
    fn from(error: ParseError) -> Self {
        DhcpctlError::ParseError(error)
    }
}

impl From<DhcpctlError> for String {
    fn from(error: DhcpctlError) -> Self {
        error.to_string()
    }
}
