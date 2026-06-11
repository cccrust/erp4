use clap::{Parser, Subcommand};
use rusqlite::Connection;
use anyhow::Result;
use crate::model::supplier;

#[derive(Parser)]
pub struct SupplierCommand {
    #[command(subcommand)]
    pub subcommand: SupplierSubcommands,
}

#[derive(Subcommand)]
pub enum SupplierSubcommands {
    /// Add a new supplier
    Add {
        name: String,
        #[arg(long)]
        contact_person: Option<String>,
        #[arg(long)]
        email: Option<String>,
        #[arg(long)]
        phone: Option<String>,
        #[arg(long)]
        address: Option<String>,
    },
    /// List all suppliers
    List,
    /// Get a supplier by ID
    Get {
        id: i64,
    },
    /// Update a supplier
    Update {
        id: i64,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        contact_person: Option<String>,
        #[arg(long)]
        email: Option<String>,
        #[arg(long)]
        phone: Option<String>,
        #[arg(long)]
        address: Option<String>,
    },
    /// Delete a supplier
    Delete {
        id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &SupplierSubcommands) -> Result<()> {
    match cmd {
        SupplierSubcommands::Add { name, contact_person, email, phone, address } => {
            let id = supplier::create_supplier(conn, name, contact_person.as_deref(), email.as_deref(), phone.as_deref(), address.as_deref())?;
            println!("Created supplier #{}: {}", id, name);
        }
        SupplierSubcommands::List => {
            let suppliers = supplier::list_suppliers(conn)?;
            if suppliers.is_empty() {
                println!("No suppliers found.");
                return Ok(());
            }
            println!("{:<4} {:<20} {:<20} {:<25} {:<15}", "ID", "Name", "Contact", "Email", "Phone");
            println!("{}", "-".repeat(90));
            for s in &suppliers {
                println!("{:<4} {:<20} {:<20} {:<25} {:<15}",
                    s.id, s.name,
                    s.contact_person.as_deref().unwrap_or(""),
                    s.email.as_deref().unwrap_or(""),
                    s.phone.as_deref().unwrap_or(""));
            }
        }
        SupplierSubcommands::Get { id } => {
            match supplier::get_supplier(conn, *id)? {
                Some(s) => {
                    println!("ID:             {}", s.id);
                    println!("Name:           {}", s.name);
                    println!("Contact Person: {}", s.contact_person.as_deref().unwrap_or("N/A"));
                    println!("Email:          {}", s.email.as_deref().unwrap_or("N/A"));
                    println!("Phone:          {}", s.phone.as_deref().unwrap_or("N/A"));
                    println!("Address:        {}", s.address.as_deref().unwrap_or("N/A"));
                    println!("Created:        {}", s.created_at);
                    println!("Updated:        {}", s.updated_at);
                }
                None => println!("Supplier #{} not found.", id),
            }
        }
        SupplierSubcommands::Update { id, name, contact_person, email, phone, address } => {
            if supplier::update_supplier(conn, *id, name.as_deref(), contact_person.as_deref(), email.as_deref(), phone.as_deref(), address.as_deref())? {
                println!("Supplier #{} updated.", id);
            } else {
                println!("Supplier #{} not found.", id);
            }
        }
        SupplierSubcommands::Delete { id } => {
            if supplier::delete_supplier(conn, *id)? {
                println!("Supplier #{} deleted.", id);
            } else {
                println!("Supplier #{} not found.", id);
            }
        }
    }
    Ok(())
}
