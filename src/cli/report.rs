use clap::{Parser, Subcommand};
use rusqlite::Connection;
use anyhow::Result;
use crate::model::{report, invoice, product};

#[derive(Parser)]
pub struct ReportCommand {
    #[command(subcommand)]
    pub subcommand: ReportSubcommands,
}

#[derive(Subcommand)]
pub enum ReportSubcommands {
    /// Sales report by product
    Sales {
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        to: Option<String>,
    },
    /// Inventory report
    Inventory,
    /// Accounts receivable aging
    Aging,
}

pub fn run(conn: &Connection, cmd: &ReportSubcommands) -> Result<()> {
    match cmd {
        ReportSubcommands::Sales { from, to } => {
            let rows = report::sales_report(conn, from.as_deref(), to.as_deref())?;
            if rows.is_empty() {
                println!("No sales data found.");
                return Ok(());
            }
            let total_qty: i64 = rows.iter().map(|r| r.total_qty).sum();
            let total_amount: f64 = rows.iter().map(|r| r.total_amount).sum();
            println!("{:<4} {:<24} {:>10} {:>14}", "ID", "Product", "Qty Sold", "Amount");
            println!("{}", "-".repeat(56));
            for r in &rows {
                println!("{:<4} {:<24} {:>10} {:>14.2}",
                    r.product_id, r.product_name, r.total_qty, r.total_amount);
            }
            println!("{}", "-".repeat(56));
            println!("{:>4} {:<24} {:>10} {:>14.2}", "", "Total", total_qty, total_amount);
        }
        ReportSubcommands::Inventory => {
            let products = product::list_products(conn, None, None)?;
            if products.is_empty() {
                println!("No products found.");
                return Ok(());
            }
            let total_value: f64 = products.iter().map(|p| p.price * p.stock as f64).sum();
            println!("{:<4} {:<24} {:<12} {:>8} {:>6} {:>14}", "ID", "Name", "SKU", "Price", "Stock", "Value");
            println!("{}", "-".repeat(80));
            for p in &products {
                let value = p.price * p.stock as f64;
                let stock_warn = if p.stock < 10 { " *" } else { "" };
                println!("{:<4} {:<24} {:<12} {:>8.2} {:>6}{} {:>14.2}",
                    p.id, p.name, p.sku, p.price, p.stock, stock_warn, value);
            }
            println!("{}", "-".repeat(80));
            println!("{:>54} {:>14.2}", "Total Inventory Value:", total_value);
        }
        ReportSubcommands::Aging => {
            let invoices = invoice::aging_report(conn)?;
            if invoices.is_empty() {
                println!("No outstanding invoices.");
                return Ok(());
            }
            let total: f64 = invoices.iter().map(|i| i.amount).sum();
            println!("{:<4} {:<15} {:<12} {:<12} {:<10} {:>10}", "ID", "Invoice#", "CustomerID", "Due Date", "Status", "Amount");
            println!("{}", "-".repeat(75));
            for inv in &invoices {
                println!("{:<4} {:<15} {:<12} {:<12} {:<10} {:>10.2}",
                    inv.id, inv.invoice_number, inv.customer_id, inv.due_date, inv.status, inv.amount);
            }
            println!("{}", "-".repeat(75));
            println!("{:>55} {:>10.2}", "Total Outstanding:", total);
        }
    }
    Ok(())
}
