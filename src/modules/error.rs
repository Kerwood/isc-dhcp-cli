use chrono::format::ParseError;
use confy::ConfyError;
use reqwest::header::InvalidHeaderValue;
use reqwest::Error as ReqwestError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DhcpctlError {
    MissingArguments,
    MissingConfigFile,
    MissingUrl,
    NotValidCIDR,
    Reqwest(ReqwestError),
    Confy(ConfyError),
    ParseError(ParseError),
    InvalidHeaderValue(InvalidHeaderValue),
    BadStatusCode(String),
}

impl Error for DhcpctlError {}

impl fmt::Display for DhcpctlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DhcpctlError::MissingArguments => write!(f, "You need to provide either an url, token or both."),
            DhcpctlError::MissingConfigFile => write!(f, "Config file not found."),
            DhcpctlError::MissingUrl => write!(f, "The URL for the ISC DHCP API is missing. Set it with 'dhcpctl config set --url https://ip-or-domain-name'."),
            DhcpctlError::NotValidCIDR => write!(f, "Not a valid CIDR."),
            DhcpctlError::Reqwest(e) => write!(f, "{}", e.to_string()),
            DhcpctlError::Confy(e) => write!(f, "[config-file] {}", e.to_string()),
            DhcpctlError::ParseError(e) => write!(f, "{}", e.to_string()),
            DhcpctlError::InvalidHeaderValue(e) => write!(f, "{}", e.to_string()),
            DhcpctlError::BadStatusCode(e) => write!(f, "{}", e),
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

impl From<InvalidHeaderValue> for DhcpctlError {
    fn from(error: InvalidHeaderValue) -> Self {
        DhcpctlError::InvalidHeaderValue(error)
    }
}

impl From<DhcpctlError> for String {
    fn from(error: DhcpctlError) -> Self {
        error.to_string()
    }
}
