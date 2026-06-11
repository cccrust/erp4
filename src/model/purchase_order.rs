use rusqlite::{params, Connection};
use anyhow::Result;
use chrono::Local;

#[derive(Debug, Clone)]
pub struct PurchaseOrder {
    pub id: i64,
    pub supplier_id: i64,
    pub order_date: String,
    pub status: String,
    pub total_amount: f64,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct PurchaseOrderItem {
    pub id: i64,
    pub purchase_order_id: i64,
    pub product_id: i64,
    pub quantity: i64,
    pub unit_price: f64,
}

pub fn create_purchase_order(conn: &Connection, supplier_id: i64, notes: Option<&str>) -> Result<i64> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let order_date = Local::now().format("%Y-%m-%d").to_string();
    conn.execute(
        "INSERT INTO purchase_orders (supplier_id, order_date, status, total_amount, notes, created_at, updated_at) VALUES (?1, ?2, 'pending', 0.0, ?3, ?4, ?5)",
        params![supplier_id, order_date, notes, now, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_purchase_orders(conn: &Connection) -> Result<Vec<PurchaseOrder>> {
    let mut stmt = conn.prepare("SELECT id, supplier_id, order_date, status, total_amount, notes, created_at, updated_at FROM purchase_orders ORDER BY id")?;
    let rows = stmt.query_map([], |row| {
        Ok(PurchaseOrder {
            id: row.get(0)?,
            supplier_id: row.get(1)?,
            order_date: row.get(2)?,
            status: row.get(3)?,
            total_amount: row.get(4)?,
            notes: row.get(5)?,
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

pub fn get_purchase_order(conn: &Connection, id: i64) -> Result<Option<PurchaseOrder>> {
    let mut stmt = conn.prepare("SELECT id, supplier_id, order_date, status, total_amount, notes, created_at, updated_at FROM purchase_orders WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(PurchaseOrder {
            id: row.get(0)?,
            supplier_id: row.get(1)?,
            order_date: row.get(2)?,
            status: row.get(3)?,
            total_amount: row.get(4)?,
            notes: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;
    match rows.next() {
        Some(Ok(po)) => Ok(Some(po)),
        _ => Ok(None),
    }
}

pub fn update_purchase_order_status(conn: &Connection, id: i64, status: &str) -> Result<bool> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let n = conn.execute(
        "UPDATE purchase_orders SET status=?1, updated_at=?2 WHERE id=?3",
        params![status, now, id],
    )?;
    Ok(n > 0)
}

pub fn delete_purchase_order(conn: &Connection, id: i64) -> Result<bool> {
    conn.execute("DELETE FROM purchase_order_items WHERE purchase_order_id = ?1", params![id])?;
    let n = conn.execute("DELETE FROM purchase_orders WHERE id = ?1", params![id])?;
    Ok(n > 0)
}

pub fn add_purchase_order_item(conn: &Connection, po_id: i64, product_id: i64, quantity: i64, unit_price: f64) -> Result<i64> {
    conn.execute(
        "INSERT INTO purchase_order_items (purchase_order_id, product_id, quantity, unit_price) VALUES (?1, ?2, ?3, ?4)",
        params![po_id, product_id, quantity, unit_price],
    )?;
    let item_id = conn.last_insert_rowid();

    let total: f64 = conn.query_row(
        "SELECT COALESCE(SUM(quantity * unit_price), 0.0) FROM purchase_order_items WHERE purchase_order_id = ?1",
        params![po_id],
        |row| row.get(0),
    )?;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "UPDATE purchase_orders SET total_amount=?1, updated_at=?2 WHERE id=?3",
        params![total, now, po_id],
    )?;
    Ok(item_id)
}

pub fn list_purchase_order_items(conn: &Connection, po_id: i64) -> Result<Vec<PurchaseOrderItem>> {
    let mut stmt = conn.prepare("SELECT id, purchase_order_id, product_id, quantity, unit_price FROM purchase_order_items WHERE purchase_order_id = ?1")?;
    let rows = stmt.query_map(params![po_id], |row| {
        Ok(PurchaseOrderItem {
            id: row.get(0)?,
            purchase_order_id: row.get(1)?,
            product_id: row.get(2)?,
            quantity: row.get(3)?,
            unit_price: row.get(4)?,
        })
    })?;
    let mut items = Vec::new();
    for row in rows {
        items.push(row?);
    }
    Ok(items)
}
