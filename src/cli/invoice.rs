use clap::{Parser, Subcommand};
use rusqlite::Connection;
use anyhow::Result;
use crate::model::invoice;

#[derive(Parser)]
pub struct InvoiceCommand {
    #[command(subcommand)]
    pub subcommand: InvoiceSubcommands,
}

#[derive(Subcommand)]
pub enum InvoiceSubcommands {
    /// Create a new invoice
    Create {
        /// Invoice number (unique)
        invoice_number: String,
        /// Order ID (optional)
        #[arg(long)]
        order_id: Option<i64>,
        /// Customer ID
        customer_id: i64,
        /// Due date (YYYY-MM-DD)
        due_date: String,
        /// Amount
        amount: f64,
        /// Notes
        #[arg(long)]
        notes: Option<String>,
    },
    /// List all invoices
    List,
    /// Get invoice details
    Get {
        id: i64,
    },
    /// Update invoice status
    UpdateStatus {
        id: i64,
        /// Status: unpaid, paid, overdue, cancelled
        status: String,
    },
    /// Delete an invoice
    Delete {
        id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &InvoiceSubcommands) -> Result<()> {
    match cmd {
        InvoiceSubcommands::Create { invoice_number, order_id, customer_id, due_date, amount, notes } => {
            let id = invoice::create_invoice(conn, invoice_number, *order_id, *customer_id, due_date, *amount, notes.as_deref())?;
            println!("Created invoice #{} ({})", id, invoice_number);
        }
        InvoiceSubcommands::List => {
            let invoices = invoice::list_invoices(conn)?;
            if invoices.is_empty() {
                println!("No invoices found.");
                return Ok(());
            }
            println!("{:<4} {:<15} {:<12} {:<12} {:<10} {:>10}", "ID", "Invoice#", "CustomerID", "Due Date", "Status", "Amount");
            println!("{}", "-".repeat(75));
            for inv in &invoices {
                println!("{:<4} {:<15} {:<12} {:<12} {:<10} {:>10.2}",
                    inv.id, inv.invoice_number, inv.customer_id, inv.due_date, inv.status, inv.amount);
            }
        }
        InvoiceSubcommands::Get { id } => {
            match invoice::get_invoice(conn, *id)? {
                Some(inv) => {
                    println!("ID:            {}", inv.id);
                    println!("Invoice #:     {}", inv.invoice_number);
                    println!("Order ID:      {}", inv.order_id.map(|x| x.to_string()).as_deref().unwrap_or("N/A".into()));
                    println!("Customer ID:   {}", inv.customer_id);
                    println!("Invoice Date:  {}", inv.invoice_date);
                    println!("Due Date:      {}", inv.due_date);
                    println!("Status:        {}", inv.status);
                    println!("Amount:        {:.2}", inv.amount);
                    println!("Notes:         {}", inv.notes.as_deref().unwrap_or("N/A"));
                    println!("Created:       {}", inv.created_at);
                    println!("Updated:       {}", inv.updated_at);
                }
                None => println!("Invoice #{} not found.", id),
            }
        }
        InvoiceSubcommands::UpdateStatus { id, status } => {
            if invoice::update_invoice_status(conn, *id, status)? {
                println!("Invoice #{} status updated to '{}'.", id, status);
            } else {
                println!("Invoice #{} not found.", id);
            }
        }
        InvoiceSubcommands::Delete { id } => {
            if invoice::delete_invoice(conn, *id)? {
                println!("Invoice #{} deleted.", id);
            } else {
                println!("Invoice #{} not found.", id);
            }
        }
    }
    Ok(())
}
