use std::collections::{HashMap, HashSet};

use super::error::DhcpctlError;
use super::reqwest_handler;
use chrono::prelude::*;
use cidr_utils::cidr::IpCidr;
use prettytable::{cell, format, row, Attr, Cell, Row, Table};
use serde::{Deserialize, Serialize};

use reqwest;

#[derive(Serialize, Deserialize, Debug)]
pub struct Lease {
    #[serde(rename = "binding-state")]
    binding_state: String,
    #[serde(rename = "client-hostname")]
    client_hostname: Option<String>,
    cltt: String,
    ends: String,
    #[serde(rename = "hardware-ethernet")]
    hardware_ethernet: String,
    ip: String,
    #[serde(rename = "next-binding-state")]
    next_binding_state: String,
    #[serde(rename = "rewind-binding-state")]
    rewind_binding_state: String,
    #[serde(rename = "set-vendor-class-identifier")]
    set_vendor_class_identifier: Option<String>,
    starts: String,
    uid: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MacVendorLookupRoot {
    pub result: MacVendorLookup,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MacVendorLookup {
    pub company: String,
}

pub async fn get_leases(cidr: String, mac_lookup: bool) -> Result<(), DhcpctlError> {
    if *&cidr.len() != 0 && !IpCidr::is_ipv4_cidr(&cidr) {
        return Err(DhcpctlError::NotValidCIDR);
    }
    let payload: Vec<Lease> = reqwest_handler::run(format!("/leases/{}", cidr).as_str()).await?;
    print_leases(payload, mac_lookup).await?;
    Ok(())
}

pub async fn search_leases(search_word: String, mac_lookup: bool) -> Result<(), DhcpctlError> {
    let payload: Vec<Lease> =
        reqwest_handler::run(format!("/leases/search/{}", search_word).as_str()).await?;
    print_leases(payload, mac_lookup).await?;
    Ok(())
}

fn table_format() -> format::TableFormat {
    format::FormatBuilder::new()
        .column_separator(' ')
        .separator(
            format::LinePosition::Title,
            format::LineSeparator::new('-', ' ', ' ', ' '),
        )
        .padding(1, 1)
        .build()
}

async fn print_leases(leases: Vec<Lease>, mac_lookup: bool) -> Result<(), DhcpctlError> {
    if leases.len() > 0 {
        let mut hash: HashMap<String, String> = HashMap::new();
        // let test = get_vendors("00:1b:21").await?;
        // println!("{:?}", macs.len());
        // println!("{:#?}", test);

        let mut table = Table::new();
        table.set_format(table_format());

        let mut row: Row = row!(b -> "MAC Address", b -> "Status", b -> "IP", b -> "Hostname", b -> "Starts", b -> "Ends", b -> "Vendor Identifier");

        if mac_lookup {
            let macs: HashSet<&str> = leases.iter().map(|x| &x.hardware_ethernet[0..8]).collect();
            hash = get_vendors(&macs).await?;
            row.add_cell(Cell::new("Mac Vendor").with_style(Attr::Bold));
        }

        table.set_titles(row);

        for lease in leases.iter() {
            let hostname = lease.client_hostname.clone().unwrap_or_default();
            let ends = DateTime::parse_from_rfc3339(&lease.ends)?.format("%Y-%m-%d %H:%M:%S");
            let starts = DateTime::parse_from_rfc3339(&lease.starts)?.format("%Y-%m-%d %H:%M:%S");
            let mut row: Row = row!(
                &lease.hardware_ethernet,
                &lease.binding_state,
                &lease.ip,
                &hostname,
                &starts,
                &ends,
                lease
                    .set_vendor_class_identifier
                    .clone()
                    .unwrap_or_default(),
            );

            if mac_lookup {
                row.insert_cell(
                    0,
                    Cell::new(
                        hash.get(&lease.hardware_ethernet[0..8])
                            .unwrap_or(&"".to_string()),
                    ),
                )
            }

            table.add_row(row);
        }
        table.printstd();
    } else {
        println!("No leases found");
    }
    Ok(())
}

async fn get_vendors(macs: &HashSet<&str>) -> Result<HashMap<String, String>, DhcpctlError> {
    let mut hash: HashMap<String, String> = HashMap::new();
    let client = reqwest::Client::new();

    for mac in macs.iter() {
        if ["2", "6", "a", "e"].contains(&&mac[1..2]) {
            hash.insert(mac.to_string(), "::randomized::".to_string());
        } else {
            let response = client
                .get(format!("https://macvendors.co/api/{}", mac))
                .send()
                .await?;
            match response.error_for_status() {
                Ok(res) => {
                    let result = match res.json::<MacVendorLookupRoot>().await {
                        Ok(x) => x.result.company,
                        Err(_) => "unknown".to_string(),
                    };
                    hash.insert(mac.to_string(), result);
                }
                Err(error) => return Err(DhcpctlError::BadStatusCode(error.to_string())),
            }
        }
    }
    Ok(hash)
}
