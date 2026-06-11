use rusqlite::{params, Connection};
use anyhow::Result;
use chrono::Local;

#[derive(Debug, Clone)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub sku: String,
    pub price: f64,
    pub stock: i64,
    pub created_at: String,
    pub updated_at: String,
}

pub fn create_product(conn: &Connection, name: &str, sku: &str, price: f64, stock: i64, description: Option<&str>) -> Result<i64> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO products (name, description, sku, price, stock, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![name, description, sku, price, stock, now, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_products(conn: &Connection) -> Result<Vec<Product>> {
    let mut stmt = conn.prepare("SELECT id, name, description, sku, price, stock, created_at, updated_at FROM products ORDER BY id")?;
    let rows = stmt.query_map([], |row| {
        Ok(Product {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            sku: row.get(3)?,
            price: row.get(4)?,
            stock: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;
    let mut items = Vec::new();
    for row in rows {
        items.push(row?);
    }
    Ok(items)
}

pub fn get_product(conn: &Connection, id: i64) -> Result<Option<Product>> {
    let mut stmt = conn.prepare("SELECT id, name, description, sku, price, stock, created_at, updated_at FROM products WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(Product {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            sku: row.get(3)?,
            price: row.get(4)?,
            stock: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;
    match rows.next() {
        Some(Ok(p)) => Ok(Some(p)),
        _ => Ok(None),
    }
}

pub fn update_product(conn: &Connection, id: i64, name: Option<&str>, sku: Option<&str>, price: Option<f64>, stock: Option<i64>, description: Option<&str>) -> Result<bool> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let existing = match get_product(conn, id)? {
        Some(p) => p,
        None => return Ok(false),
    };
    let name = name.unwrap_or(&existing.name);
    let sku = sku.unwrap_or(&existing.sku);
    let price = price.unwrap_or(existing.price);
    let stock = stock.unwrap_or(existing.stock);
    let description = description.or(existing.description.as_deref());
    conn.execute(
        "UPDATE products SET name=?1, description=?2, sku=?3, price=?4, stock=?5, updated_at=?6 WHERE id=?7",
        params![name, description, sku, price, stock, now, id],
    )?;
    Ok(true)
}

pub fn delete_product(conn: &Connection, id: i64) -> Result<bool> {
    let n = conn.execute("DELETE FROM products WHERE id = ?1", params![id])?;
    Ok(n > 0)
}
