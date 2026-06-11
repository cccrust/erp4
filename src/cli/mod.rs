use clap::{Parser, Subcommand};

pub mod customer;
pub mod product;
pub mod supplier;
pub mod order;
pub mod purchase_order;
pub mod invoice;
pub mod init;

pub use customer::CustomerCommand;
pub use product::ProductCommand;
pub use supplier::SupplierCommand;
pub use order::OrderCommand;
pub use purchase_order::PurchaseOrderCommand;
pub use invoice::InvoiceCommand;

#[derive(Parser)]
#[command(name = "erp4", version, about = "ERP4 - Enterprise Resource Planning CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize the ERP database
    Init,
    /// Manage customers
    Customer(CustomerCommand),
    /// Manage products
    Product(ProductCommand),
    /// Manage suppliers
    Supplier(SupplierCommand),
    /// Manage orders
    Order(OrderCommand),
    /// Manage purchase orders
    #[command(name = "purchase-order")]
    PurchaseOrder(PurchaseOrderCommand),
    /// Manage invoices
    Invoice(InvoiceCommand),
}
