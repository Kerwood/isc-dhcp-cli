use super::reqwest_handler;
use chrono::prelude::*;
use cidr_utils::cidr::IpCidr;
use prettytable::{cell, format, row, Table};
use serde::{Deserialize, Serialize};
use std::error::Error;

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

pub async fn get_leases(cidr: String) -> Result<(), Box<dyn Error>> {
    if *&cidr.len() != 0 && !IpCidr::is_ipv4_cidr(&cidr) {
        return Err("Not a valid CIDR.".into());
    }
    let payload: Vec<Lease> = reqwest_handler::run(format!("/leases/{}", cidr).as_str()).await?;
    print_leases(payload)?;
    Ok(())
}

pub async fn search_leases(search_word: String) -> Result<(), Box<dyn Error>> {
    let payload: Vec<Lease> =
        reqwest_handler::run(format!("/leases/search/{}", search_word).as_str()).await?;
    print_leases(payload)?;
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

fn print_leases(leases: Vec<Lease>) -> Result<(), Box<dyn Error>> {
    let mut table = Table::new();
    table.set_format(table_format());
    table.set_titles(row!(b -> "MAC Address", b -> "Status", b -> "IP", b -> "Hostname", b -> "Starts", b -> "Ends", b -> "Vendor Identifier"));
    for lease in leases.iter() {
        let hostname = lease.client_hostname.clone().unwrap_or_default();
        let ends = DateTime::parse_from_rfc3339(&lease.ends)?.format("%Y-%m-%d %H:%M:%S");
        let starts = DateTime::parse_from_rfc3339(&lease.starts)?.format("%Y-%m-%d %H:%M:%S");
        table.add_row(row!(
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
        ));
    }
    table.printstd();
    Ok(())
}
