use super::reqwest_handler;
use colored::*;
use prettytable::{cell, format, row, Attr, Cell, Row, Table};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Scope {
    ip: String,
    subnet: String,
    range: Range,
    options: Options,
    #[serde(rename = "next-server")]
    next_server: Option<String>,
    #[serde(rename = "default-lease-time")]
    default_lease_time: Option<String>,
    #[serde(rename = "max-lease-time")]
    max_lease_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Range {
    start: String,
    end: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Options {
    #[serde(rename = "subnet-mask")]
    subnet_mask: String,
    #[serde(rename = "broadcast-address")]
    broadcast_address: String,
    routers: String,
    #[serde(rename = "tftp-server-name")]
    tftp_server_name: Option<String>,
    #[serde(rename = "bootfile-name")]
    bootfile_name: Option<String>,
    #[serde(rename = "domain-name")]
    domain_name: Option<String>,
    #[serde(rename = "domain-name-servers")]
    domain_name_servers: Option<Vec<String>>,
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

pub async fn list_scopes(pxe: bool, dns: bool) -> Result<(), Box<dyn Error>> {
    let payload: Vec<Scope> = reqwest_handler::run("/config/scopes").await?;

    let mut table = Table::new();
    table.set_format(table_format());

    let mut row: Row = row!(b -> "Subnet ID", b -> "Subnet Mask", b -> "Scope Start", b -> "Scope End", b -> "Gateway");

    if dns {
        row.add_cell(Cell::new("DNS Servers").with_style(Attr::Bold));
    }

    if pxe {
        row.add_cell(Cell::new("TFTP Server").with_style(Attr::Bold));
        row.add_cell(Cell::new("Bootfile").with_style(Attr::Bold));
        row.add_cell(Cell::new("Next Server").with_style(Attr::Bold));
    }

    table.set_titles(row);

    for scope in payload.iter() {
        let mut row: Row = row!(
            &scope.ip,
            &scope.subnet,
            &scope.range.start,
            &scope.range.end,
            &scope.options.routers,
        );

        if dns {
            row.add_cell(Cell::new(
                &scope
                    .options
                    .domain_name_servers
                    .clone()
                    .unwrap_or_default()
                    .join(","),
            ));
        }

        if pxe {
            row.add_cell(Cell::new(
                &scope.options.tftp_server_name.clone().unwrap_or_default(),
            ));
            row.add_cell(Cell::new(
                &scope.options.bootfile_name.clone().unwrap_or_default(),
            ));
            row.add_cell(Cell::new(&scope.next_server.clone().unwrap_or_default()));
        }
        table.add_row(row);
    }
    table.printstd();
    Ok(())
}

pub async fn get_scope(subnet_id: &str) -> Result<(), Box<dyn Error>> {
    let mut payload: Vec<Scope> = reqwest_handler::run("/config/scopes").await?;

    payload.retain(|f| f.ip == subnet_id);

    if payload.len() == 0 {
        println!("No subnet found by that ID");
    }

    for scope in payload.iter() {
        println!("{} {}", "Network ID:".bold(), &scope.ip);
        println!("{} {}", "Subnet Mask:".bold(), &scope.subnet);
        println!(
            "{} {} - {}",
            "Scope Range:".bold(),
            &scope.range.start,
            &scope.range.end
        );
        println!("{} {}", "Gateway:".bold(), &scope.options.routers);

        if scope.options.tftp_server_name.is_some()
            || scope.options.bootfile_name.is_some()
            || scope.next_server.is_some()
        {
            println!(
                "\n{} {}",
                "TFTP Server:".bold(),
                scope
                    .options
                    .tftp_server_name
                    .clone()
                    .unwrap_or("Not set".to_string())
            );
            println!(
                "{} {}",
                "Bootfile:".bold(),
                scope
                    .options
                    .bootfile_name
                    .clone()
                    .unwrap_or("Not set".to_string())
            );
            println!(
                "{} {}",
                "Next Server:".bold(),
                scope.next_server.clone().unwrap_or("Not set".to_string())
            );
        }

        if scope.options.domain_name_servers.is_some() {
            println!(
                "\n{} {}",
                "DNS Servers:".bold(),
                scope.options.domain_name_servers.clone().unwrap().join(",")
            );
        }

        if scope.options.domain_name.is_some() {
            println!(
                "\n{} {}",
                "Domain Name:".bold(),
                scope.options.domain_name.clone().unwrap()
            );
        }
        if scope.default_lease_time.is_some() {
            println!(
                "\n{} {}",
                "Default Lease Time:".bold(),
                scope.default_lease_time.clone().unwrap()
            );
        }
        if scope.max_lease_time.is_some() {
            println!(
                "\n{} {}",
                "Max Lease Time:".bold(),
                scope.max_lease_time.clone().unwrap()
            );
        }
    }

    Ok(())
}
