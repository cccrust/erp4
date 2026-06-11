use clap::{Parser, Subcommand};
use rusqlite::Connection;
use anyhow::Result;
use crate::model::product;

#[derive(Parser)]
pub struct ProductCommand {
    #[command(subcommand)]
    pub subcommand: ProductSubcommands,
}

#[derive(Subcommand)]
pub enum ProductSubcommands {
    /// Add a new product
    Add {
        /// Product name
        name: String,
        /// SKU (unique)
        sku: String,
        /// Price
        price: f64,
        /// Stock quantity
        #[arg(long, default_value_t = 0)]
        stock: i64,
        /// Description
        #[arg(long)]
        description: Option<String>,
    },
    /// List all products
    List,
    /// Get a product by ID
    Get {
        id: i64,
    },
    /// Update a product
    Update {
        id: i64,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        sku: Option<String>,
        #[arg(long)]
        price: Option<f64>,
        #[arg(long)]
        stock: Option<i64>,
        #[arg(long)]
        description: Option<String>,
    },
    /// Delete a product
    Delete {
        id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &ProductSubcommands) -> Result<()> {
    match cmd {
        ProductSubcommands::Add { name, sku, price, stock, description } => {
            let id = product::create_product(conn, name, sku, *price, *stock, description.as_deref())?;
            println!("Created product #{}: {} ({})", id, name, sku);
        }
        ProductSubcommands::List => {
            let products = product::list_products(conn)?;
            if products.is_empty() {
                println!("No products found.");
                return Ok(());
            }
            println!("{:<4} {:<20} {:<12} {:>8} {:>6} {:>6}", "ID", "Name", "SKU", "Price", "Stock", "");
            println!("{}", "-".repeat(80));
            for p in &products {
                println!("{:<4} {:<20} {:<12} {:>8.2} {:>6}",
                    p.id, p.name, p.sku, p.price, p.stock);
            }
        }
        ProductSubcommands::Get { id } => {
            match product::get_product(conn, *id)? {
                Some(p) => {
                    println!("ID:          {}", p.id);
                    println!("Name:        {}", p.name);
                    println!("SKU:         {}", p.sku);
                    println!("Price:       {:.2}", p.price);
                    println!("Stock:       {}", p.stock);
                    println!("Description: {}", p.description.as_deref().unwrap_or("N/A"));
                    println!("Created:     {}", p.created_at);
                    println!("Updated:     {}", p.updated_at);
                }
                None => println!("Product #{} not found.", id),
            }
        }
        ProductSubcommands::Update { id, name, sku, price, stock, description } => {
            if product::update_product(conn, *id, name.as_deref(), sku.as_deref(), *price, *stock, description.as_deref())? {
                println!("Product #{} updated.", id);
            } else {
                println!("Product #{} not found.", id);
            }
        }
        ProductSubcommands::Delete { id } => {
            if product::delete_product(conn, *id)? {
                println!("Product #{} deleted.", id);
            } else {
                println!("Product #{} not found.", id);
            }
        }
    }
    Ok(())
}
