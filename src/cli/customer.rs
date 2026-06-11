use clap::{Parser, Subcommand};
use rusqlite::Connection;
use anyhow::Result;
use crate::model::customer;

#[derive(Parser)]
pub struct CustomerCommand {
    #[command(subcommand)]
    pub subcommand: CustomerSubcommands,
}

#[derive(Subcommand)]
pub enum CustomerSubcommands {
    /// Add a new customer
    Add {
        /// Customer name
        name: String,
        /// Email address
        #[arg(long)]
        email: Option<String>,
        /// Phone number
        #[arg(long)]
        phone: Option<String>,
        /// Address
        #[arg(long)]
        address: Option<String>,
    },
    /// List all customers
    List,
    /// Get a customer by ID
    Get {
        /// Customer ID
        id: i64,
    },
    /// Update a customer
    Update {
        /// Customer ID
        id: i64,
        /// New name
        #[arg(long)]
        name: Option<String>,
        /// New email
        #[arg(long)]
        email: Option<String>,
        /// New phone
        #[arg(long)]
        phone: Option<String>,
        /// New address
        #[arg(long)]
        address: Option<String>,
    },
    /// Delete a customer
    Delete {
        /// Customer ID
        id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &CustomerSubcommands) -> Result<()> {
    match cmd {
        CustomerSubcommands::Add { name, email, phone, address } => {
            let id = customer::create_customer(conn, name, email.as_deref(), phone.as_deref(), address.as_deref())?;
            println!("Created customer #{}: {}", id, name);
        }
        CustomerSubcommands::List => {
            let customers = customer::list_customers(conn)?;
            if customers.is_empty() {
                println!("No customers found.");
                return Ok(());
            }
            println!("{:<4} {:<20} {:<25} {:<15} {:<30}", "ID", "Name", "Email", "Phone", "Address");
            println!("{}", "-".repeat(100));
            for c in &customers {
                println!("{:<4} {:<20} {:<25} {:<15} {:<30}",
                    c.id, c.name,
                    c.email.as_deref().unwrap_or(""),
                    c.phone.as_deref().unwrap_or(""),
                    c.address.as_deref().unwrap_or(""));
            }
        }
        CustomerSubcommands::Get { id } => {
            match customer::get_customer(conn, *id)? {
                Some(c) => {
                    println!("ID:        {}", c.id);
                    println!("Name:      {}", c.name);
                    println!("Email:     {}", c.email.as_deref().unwrap_or("N/A"));
                    println!("Phone:     {}", c.phone.as_deref().unwrap_or("N/A"));
                    println!("Address:   {}", c.address.as_deref().unwrap_or("N/A"));
                    println!("Created:   {}", c.created_at);
                    println!("Updated:   {}", c.updated_at);
                }
                None => println!("Customer #{} not found.", id),
            }
        }
        CustomerSubcommands::Update { id, name, email, phone, address } => {
            if customer::update_customer(conn, *id, name.as_deref(), email.as_deref(), phone.as_deref(), address.as_deref())? {
                println!("Customer #{} updated.", id);
            } else {
                println!("Customer #{} not found.", id);
            }
        }
        CustomerSubcommands::Delete { id } => {
            if customer::delete_customer(conn, *id)? {
                println!("Customer #{} deleted.", id);
            } else {
                println!("Customer #{} not found.", id);
            }
        }
    }
    Ok(())
}
