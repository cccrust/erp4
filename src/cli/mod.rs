use clap::{Parser, Subcommand};

pub mod customer;
pub mod fmt;
pub mod init;
pub mod invoice;
pub mod order;
pub mod product;
pub mod purchase_order;
pub mod report;
pub mod supplier;

pub use customer::CustomerCommand;
pub use invoice::InvoiceCommand;
pub use order::OrderCommand;
pub use product::ProductCommand;
pub use purchase_order::PurchaseOrderCommand;
pub use report::ReportCommand;
pub use supplier::SupplierCommand;

#[derive(Parser)]
#[command(
    name = "erp4",
    version,
    about = "ERP4 - Enterprise Resource Planning CLI"
)]
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
