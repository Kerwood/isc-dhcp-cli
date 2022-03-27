mod modules;
use modules::{config, globals, leases, scopes};
use structopt::StructOpt;

use crate::modules::error::DhcpctlError;
const APP_NAME: &str = "dhcpctl";
const CONFIG_NAME: &str = "config";

#[derive(StructOpt, Debug)]
#[structopt(
    name = "dhcpctl",
    author = "Author Patrick Kerwood <patrick@kerwood.dk>"
)]
struct Dhcpctl {
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(StructOpt, Debug)]
enum Cmd {
    #[structopt(about = "Set URL and/or auth token.")]
    Config(Config),

    #[structopt(name = "globals", about = "Get DHCP global configuration.")]
    Globals {},

    #[structopt(name = "scopes", about = "Get all DHCP scopes.")]
    Scopes(ScopeType),

    #[structopt(name = "leases", about = "Get all DHCP leases.")]
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

        #[structopt(
            long,
            short,
            takes_value = false,
            help = "Look up the vendor on MAC addresses."
        )]
        mac_lookup: bool,
    },
    #[structopt(
        about = "Search for leases in the 'client-hostname', 'hardware-ethernet' and 'set-vendor-class-identifier' properties."
    )]
    Search {
        #[structopt(help = "The string to search for.")]
        string: String,

        #[structopt(
            long,
            short,
            takes_value = false,
            help = "Look up the vendor on MAC addresses."
        )]
        mac_lookup: bool,
    },
}

#[derive(StructOpt, Debug)]
enum Config {
    #[structopt(about = "Set configuration.")]
    Set {
        #[structopt(short, long, help = "Set the URL of the ISC DHCP API.", global = true)]
        url: Option<String>,

        #[structopt(
            short,
            long,
            global = true,
            help = "API authentication token. This will be added as a header eg. 'authorization: xxxxx'."
        )]
        token: Option<String>,
    },

    #[structopt(about = "List configuration.")]
    List {},
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Dhcpctl::from_args();

    match args.cmd {
        Cmd::Config(config_type) => match config_type {
            Config::Set { url, token } => {
                if url.is_none() && token.is_none() {
                    return Err(DhcpctlError::MissingArguments.to_string());
                }
                if url.is_some() {
                    config::set_api_url(&url.expect("Error unwrapping url option"))?;
                }
                if token.is_some() {
                    config::set_auth_token(&token.expect("Error unwrapping token option"))?;
                }
            }

            Config::List {} => {
                config::print_config()?;
            }
        },

        Cmd::Globals {} => {
            globals::list_globals().await?;
        }

        Cmd::Scopes(scope_type) => match scope_type {
            ScopeType::List { pxe, dns } => {
                scopes::list_scopes(pxe, dns).await?;
            }
            ScopeType::Get { subnet_id } => {
                scopes::get_scope(&subnet_id).await?;
            }
        },

        Cmd::Leases(lease_type) => match lease_type {
            LeaseType::List { cidr, mac_lookup } => {
                let cidr = cidr.unwrap_or_default();
                leases::get_leases(cidr, mac_lookup).await?;
            }
            LeaseType::Search { string, mac_lookup } => {
                leases::search_leases(string, mac_lookup).await?;
            }
        },
    }
    Ok(())
}
