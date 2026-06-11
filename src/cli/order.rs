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
    /// Create a new order
    Create {
        /// Customer ID
        customer_id: i64,
        /// Notes
        #[arg(long)]
        notes: Option<String>,
    },
    /// List all orders
    List,
    /// Get order details
    Get {
        id: i64,
    },
    /// Update order status
    UpdateStatus {
        id: i64,
        /// Status: pending, confirmed, shipped, delivered, cancelled
        status: String,
    },
    /// Delete an order
    Delete {
        id: i64,
    },
    /// Add item to order
    AddItem {
        /// Order ID
        order_id: i64,
        /// Product ID
        product_id: i64,
        /// Quantity
        quantity: i64,
        /// Unit price (overrides product price)
        #[arg(long)]
        unit_price: Option<f64>,
    },
    /// List items in an order
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
        OrderSubcommands::List => {
            let orders = order::list_orders(conn)?;
            if orders.is_empty() {
                println!("No orders found.");
                return Ok(());
            }
            println!("{:<4} {:<12} {:<12} {:<12} {:>10} {:>10}", "ID", "CustomerID", "Date", "Status", "Amount", "");
            println!("{}", "-".repeat(80));
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
            if order::update_order_status(conn, *id, status)? {
                println!("Order #{} status updated to '{}'.", id, status);
            } else {
                println!("Order #{} not found.", id);
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
            let item_id = order::add_order_item(conn, *order_id, *product_id, *quantity, price)?;
            println!("Added item #{} to order #{}", item_id, order_id);
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
