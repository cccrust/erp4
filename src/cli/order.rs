use clap::{Parser, Subcommand};
use rusqlite::Connection;
use anyhow::Result;
use crate::model::order;

#[derive(Parser)]
pub struct OrderCommand {
    #[command(subcommand)]
    pub subcommand: OrderSubcommands,
}

#[derive(Subcommand)]
pub enum OrderSubcommands {
    Create {
        customer_id: i64,
        #[arg(long)]
        notes: Option<String>,
    },
    List {
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        customer_id: Option<i64>,
    },
    Get {
        id: i64,
    },
    UpdateStatus {
        id: i64,
        status: String,
    },
    Delete {
        id: i64,
    },
    AddItem {
        order_id: i64,
        product_id: i64,
        quantity: i64,
        #[arg(long)]
        unit_price: Option<f64>,
    },
    Items {
        order_id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &OrderSubcommands) -> Result<()> {
    match cmd {
        OrderSubcommands::Create { customer_id, notes } => {
            let id = order::create_order(conn, *customer_id, notes.as_deref())?;
            println!("Created order #{} for customer #{}", id, customer_id);
        }
        OrderSubcommands::List { status, customer_id } => {
            let orders = order::list_orders(conn, status.as_deref(), *customer_id)?;
            if orders.is_empty() {
                println!("No orders found.");
                return Ok(());
            }
            println!("{:<4} {:<12} {:<12} {:<12} {:>10}", "ID", "CustomerID", "Date", "Status", "Amount");
            println!("{}", "-".repeat(60));
            for o in &orders {
                println!("{:<4} {:<12} {:<12} {:<12} {:>10.2}",
                    o.id, o.customer_id, o.order_date, o.status, o.total_amount);
            }
        }
        OrderSubcommands::Get { id } => {
            match order::get_order(conn, *id)? {
                Some(o) => {
                    println!("ID:           {}", o.id);
                    println!("Customer ID:  {}", o.customer_id);
                    println!("Order Date:   {}", o.order_date);
                    println!("Status:       {}", o.status);
                    println!("Total Amount: {:.2}", o.total_amount);
                    println!("Notes:        {}", o.notes.as_deref().unwrap_or("N/A"));
                    println!("Created:      {}", o.created_at);
                    println!("Updated:      {}", o.updated_at);

                    let items = order::list_order_items(conn, o.id)?;
                    if !items.is_empty() {
                        println!("\nItems:");
                        println!("{:<4} {:<12} {:>8} {:>10}", "ID", "ProductID", "Qty", "UnitPrice");
                        println!("{}", "-".repeat(40));
                        for item in &items {
                            println!("{:<4} {:<12} {:>8} {:>10.2}", item.id, item.product_id, item.quantity, item.unit_price);
                        }
                    }
                }
                None => println!("Order #{} not found.", id),
            }
        }
        OrderSubcommands::UpdateStatus { id, status } => {
            match order::update_order_status(conn, *id, status) {
                Ok(true) => println!("Order #{} status updated to '{}'.", id, status),
                Ok(false) => println!("Order #{} not found.", id),
                Err(e) => println!("Error: {}", e),
            }
        }
        OrderSubcommands::Delete { id } => {
            if order::delete_order(conn, *id)? {
                println!("Order #{} deleted.", id);
            } else {
                println!("Order #{} not found.", id);
            }
        }
        OrderSubcommands::AddItem { order_id, product_id, quantity, unit_price } => {
            let price = match unit_price {
                Some(p) => *p,
                None => {
                    let prod = crate::model::product::get_product(conn, *product_id)?
                        .ok_or_else(|| anyhow::anyhow!("Product #{} not found.", product_id))?;
                    prod.price
                }
            };
            match order::add_order_item(conn, *order_id, *product_id, *quantity, price) {
                Ok(item_id) => println!("Added item #{} to order #{}", item_id, order_id),
                Err(e) => println!("Error: {}", e),
            }
        }
        OrderSubcommands::Items { order_id } => {
            let items = order::list_order_items(conn, *order_id)?;
            if items.is_empty() {
                println!("No items in order #{}.", order_id);
                return Ok(());
            }
            println!("{:<4} {:<12} {:>8} {:>10}", "ID", "ProductID", "Qty", "UnitPrice");
            println!("{}", "-".repeat(40));
            for item in &items {
                println!("{:<4} {:<12} {:>8} {:>10.2}", item.id, item.product_id, item.quantity, item.unit_price);
            }
        }
    }
    Ok(())
}
