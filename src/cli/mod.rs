use clap::{Parser, Subcommand};

pub mod customer;
pub mod product;
pub mod supplier;
pub mod order;
pub mod purchase_order;
pub mod invoice;
pub mod report;
pub mod init;

pub use customer::CustomerCommand;
pub use product::ProductCommand;
pub use supplier::SupplierCommand;
pub use order::OrderCommand;
pub use purchase_order::PurchaseOrderCommand;
pub use invoice::InvoiceCommand;
pub use report::ReportCommand;

#[derive(Parser)]
#[command(name = "erp4", version, about = "ERP4 - Enterprise Resource Planning CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Customer(CustomerCommand),
    Product(ProductCommand),
    Supplier(SupplierCommand),
    Order(OrderCommand),
    #[command(name = "purchase-order")]
    PurchaseOrder(PurchaseOrderCommand),
    Invoice(InvoiceCommand),
    Report(ReportCommand),
}
