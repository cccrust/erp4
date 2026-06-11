use clap::{Parser, Subcommand};
use rusqlite::Connection;
use anyhow::Result;
use crate::model::purchase_order;

#[derive(Parser)]
pub struct PurchaseOrderCommand {
    #[command(subcommand)]
    pub subcommand: PurchaseOrderSubcommands,
}

#[derive(Subcommand)]
pub enum PurchaseOrderSubcommands {
    /// Create a new purchase order
    Create {
        /// Supplier ID
        supplier_id: i64,
        /// Notes
        #[arg(long)]
        notes: Option<String>,
    },
    /// List all purchase orders
    List,
    /// Get purchase order details
    Get {
        id: i64,
    },
    /// Update purchase order status
    UpdateStatus {
        id: i64,
        /// Status: pending, approved, received, cancelled
        status: String,
    },
    /// Delete a purchase order
    Delete {
        id: i64,
    },
    /// Add item to purchase order
    AddItem {
        /// Purchase order ID
        po_id: i64,
        /// Product ID
        product_id: i64,
        /// Quantity
        quantity: i64,
        /// Unit price
        #[arg(long)]
        unit_price: Option<f64>,
    },
    /// List items in a purchase order
    Items {
        po_id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &PurchaseOrderSubcommands) -> Result<()> {
    match cmd {
        PurchaseOrderSubcommands::Create { supplier_id, notes } => {
            let id = purchase_order::create_purchase_order(conn, *supplier_id, notes.as_deref())?;
            println!("Created purchase order #{} for supplier #{}", id, supplier_id);
        }
        PurchaseOrderSubcommands::List => {
            let pos = purchase_order::list_purchase_orders(conn)?;
            if pos.is_empty() {
                println!("No purchase orders found.");
                return Ok(());
            }
            println!("{:<4} {:<12} {:<12} {:<12} {:>10}", "ID", "SupplierID", "Date", "Status", "Amount");
            println!("{}", "-".repeat(60));
            for po in &pos {
                println!("{:<4} {:<12} {:<12} {:<12} {:>10.2}",
                    po.id, po.supplier_id, po.order_date, po.status, po.total_amount);
            }
        }
        PurchaseOrderSubcommands::Get { id } => {
            match purchase_order::get_purchase_order(conn, *id)? {
                Some(po) => {
                    println!("ID:           {}", po.id);
                    println!("Supplier ID:  {}", po.supplier_id);
                    println!("Order Date:   {}", po.order_date);
                    println!("Status:       {}", po.status);
                    println!("Total Amount: {:.2}", po.total_amount);
                    println!("Notes:        {}", po.notes.as_deref().unwrap_or("N/A"));
                    println!("Created:      {}", po.created_at);
                    println!("Updated:      {}", po.updated_at);

                    let items = purchase_order::list_purchase_order_items(conn, po.id)?;
                    if !items.is_empty() {
                        println!("\nItems:");
                        println!("{:<4} {:<12} {:>8} {:>10}", "ID", "ProductID", "Qty", "UnitPrice");
                        println!("{}", "-".repeat(40));
                        for item in &items {
                            println!("{:<4} {:<12} {:>8} {:>10.2}", item.id, item.product_id, item.quantity, item.unit_price);
                        }
                    }
                }
                None => println!("Purchase order #{} not found.", id),
            }
        }
        PurchaseOrderSubcommands::UpdateStatus { id, status } => {
            if purchase_order::update_purchase_order_status(conn, *id, status)? {
                println!("Purchase order #{} status updated to '{}'.", id, status);
            } else {
                println!("Purchase order #{} not found.", id);
            }
        }
        PurchaseOrderSubcommands::Delete { id } => {
            if purchase_order::delete_purchase_order(conn, *id)? {
                println!("Purchase order #{} deleted.", id);
            } else {
                println!("Purchase order #{} not found.", id);
            }
        }
        PurchaseOrderSubcommands::AddItem { po_id, product_id, quantity, unit_price } => {
            let price = match unit_price {
                Some(p) => *p,
                None => {
                    let prod = crate::model::product::get_product(conn, *product_id)?
                        .ok_or_else(|| anyhow::anyhow!("Product #{} not found.", product_id))?;
                    prod.price
                }
            };
            let item_id = purchase_order::add_purchase_order_item(conn, *po_id, *product_id, *quantity, price)?;
            println!("Added item #{} to purchase order #{}", item_id, po_id);
        }
        PurchaseOrderSubcommands::Items { po_id } => {
            let items = purchase_order::list_purchase_order_items(conn, *po_id)?;
            if items.is_empty() {
                println!("No items in purchase order #{}.", po_id);
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
