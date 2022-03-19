mod modules;
use modules::{config, globals, leases, scopes};
use structopt::StructOpt;

const APP_NAME: &str = "dhcpctl";
const CONFIG_NAME: &str = "config";

#[derive(StructOpt, Debug)]
#[structopt(
    name = "dhcpctl",
    author = "Author Patrick Kerwood <patrick@kerwood.dk>"
)]

enum Dhcpctl {
    #[structopt(name = "set", about = "Set CLI configuration.")]
    Set(SetType),

    #[structopt(name = "get", about = "Get CLI configuration.")]
    Get(GetType),

    #[structopt(name = "globals", about = "Get global configuration.")]
    Globals {},

    #[structopt(name = "scopes", about = "Get all DHCP scopes.")]
    Scopes(ScopeType),

    #[structopt(name = "leases", about = "Get all leases.")]
    Leases(LeaseType),
}

#[derive(StructOpt, Debug)]
enum ScopeType {
    #[structopt(about = "List all scopes.")]
    List {
        #[structopt(long, short, takes_value = false, help = "Include PXE options.")]
        pxe: bool,

        #[structopt(long, short, takes_value = false, help = "Include DNS servers.")]
        dns: bool,
    },
    #[structopt(about = "Get a specific scope.")]
    Get {
        #[structopt(help = "The string to search for.")]
        subnet_id: String,
    },
}

#[derive(StructOpt, Debug)]
enum LeaseType {
    #[structopt(about = "List all active leases.")]
    List {
        #[structopt(help = "Specific CIDR or IP, Eg. 10.3.0.0/24 or 10.3.0.120")]
        cidr: Option<String>,
    },
    #[structopt(
        about = "Search for leases in the 'client-hostname', 'hardware-ethernet' and 'set-vendor-class-identifier' properties."
    )]
    Search {
        #[structopt(help = "The string to search for.")]
        string: String,
    },
}

// ##########################

#[derive(StructOpt, Debug)]
enum SetType {
    #[structopt(about = "Set configuration")]
    Config {
        #[structopt(short, long, help = "Set the URL of the ISC DHCP API.")]
        url: Option<String>,
        #[structopt(
            short,
            long,
            help = "API authentication token. This will be added as a header eg. 'authorization: xxxxx'"
        )]
        token: Option<String>,
    },
}

#[derive(StructOpt, Debug)]
enum GetType {
    #[structopt(about = "Set configuration")]
    Config {},
}

#[tokio::main]
async fn main() -> Result<(), String> {
    if let Err(_) = config::check_if_conf_exists() {
        std::process::exit(1)
    }

    match Dhcpctl::from_args() {
        Dhcpctl::Set(config_type) => match config_type {
            SetType::Config { url, token } => {
                if url.is_some() {
                    config::set_api_url(&url.unwrap())?;
                }
                if token.is_some() {
                    config::set_auth_token(&token.unwrap())?;
                }
            }
        },

        Dhcpctl::Get(config_type) => match config_type {
            GetType::Config {} => {
                println!("API URL: {}", config::get_api_url()?);
                println!("Auth token: {}", config::get_auth_token()?);
            }
        },

        Dhcpctl::Globals {} => {
            globals::list_globals().await?;
        }

        Dhcpctl::Scopes(scope_type) => match scope_type {
            ScopeType::List { pxe, dns } => {
                scopes::list_scopes(pxe, dns).await?;
            }
            ScopeType::Get { subnet_id } => {
                scopes::get_scope(&subnet_id).await?;
            }
        },

        Dhcpctl::Leases(lease_type) => match lease_type {
            LeaseType::List { cidr } => {
                let cidr = cidr.unwrap_or_default();
                leases::get_leases(cidr).await?;
            }
            LeaseType::Search { string } => {
                leases::search_leases(string).await?;
            }
        },
    }
    Ok(())
}
