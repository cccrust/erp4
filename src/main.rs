#![allow(dead_code, unused)]

mod cli;
mod db;
mod model;

use clap::Parser;
use rusqlite::Connection;
use anyhow::Result;
use std::path::PathBuf;

fn get_db_path() -> PathBuf {
    let path = std::env::var("ERP4_DB").unwrap_or_else(|_| "erp4.db".to_string());
    PathBuf::from(path)
}

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    let db_path = get_db_path();
    let conn = Connection::open(&db_path)?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    match &cli.command {
        cli::Commands::Init => {
            cli::init::run(&conn)?;
        }
        cli::Commands::Customer(cmd) => {
            cli::customer::run(&conn, &cmd.subcommand)?;
        }
        cli::Commands::Product(cmd) => {
            cli::product::run(&conn, &cmd.subcommand)?;
        }
        cli::Commands::Supplier(cmd) => {
            cli::supplier::run(&conn, &cmd.subcommand)?;
        }
        cli::Commands::Order(cmd) => {
            cli::order::run(&conn, &cmd.subcommand)?;
        }
        cli::Commands::PurchaseOrder(cmd) => {
            cli::purchase_order::run(&conn, &cmd.subcommand)?;
        }
        cli::Commands::Invoice(cmd) => {
            cli::invoice::run(&conn, &cmd.subcommand)?;
        }
    }

    Ok(())
}
