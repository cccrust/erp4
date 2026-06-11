use rusqlite::{params, Connection};
use anyhow::Result;
use chrono::Local;

#[derive(Debug, Clone)]
pub struct Invoice {
    pub id: i64,
    pub invoice_number: String,
    pub order_id: Option<i64>,
    pub customer_id: i64,
    pub invoice_date: String,
    pub due_date: String,
    pub status: String,
    pub amount: f64,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn create_invoice(conn: &Connection, invoice_number: &str, order_id: Option<i64>, customer_id: i64, due_date: &str, amount: f64, notes: Option<&str>) -> Result<i64> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let invoice_date = Local::now().format("%Y-%m-%d").to_string();
    conn.execute(
        "INSERT INTO invoices (invoice_number, order_id, customer_id, invoice_date, due_date, status, amount, notes, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, 'unpaid', ?6, ?7, ?8, ?9)",
        params![invoice_number, order_id, customer_id, invoice_date, due_date, amount, notes, now, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_invoices(conn: &Connection) -> Result<Vec<Invoice>> {
    let mut stmt = conn.prepare("SELECT id, invoice_number, order_id, customer_id, invoice_date, due_date, status, amount, notes, created_at, updated_at FROM invoices ORDER BY id")?;
    let rows = stmt.query_map([], |row| {
        Ok(Invoice {
            id: row.get(0)?,
            invoice_number: row.get(1)?,
            order_id: row.get(2)?,
            customer_id: row.get(3)?,
            invoice_date: row.get(4)?,
            due_date: row.get(5)?,
            status: row.get(6)?,
            amount: row.get(7)?,
            notes: row.get(8)?,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        })
    })?;
    let mut items = Vec::new();
    for row in rows {
        items.push(row?);
    }
    Ok(items)
}

pub fn get_invoice(conn: &Connection, id: i64) -> Result<Option<Invoice>> {
    let mut stmt = conn.prepare("SELECT id, invoice_number, order_id, customer_id, invoice_date, due_date, status, amount, notes, created_at, updated_at FROM invoices WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(Invoice {
            id: row.get(0)?,
            invoice_number: row.get(1)?,
            order_id: row.get(2)?,
            customer_id: row.get(3)?,
            invoice_date: row.get(4)?,
            due_date: row.get(5)?,
            status: row.get(6)?,
            amount: row.get(7)?,
            notes: row.get(8)?,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        })
    })?;
    match rows.next() {
        Some(Ok(inv)) => Ok(Some(inv)),
        _ => Ok(None),
    }
}

pub fn update_invoice_status(conn: &Connection, id: i64, status: &str) -> Result<bool> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let n = conn.execute(
        "UPDATE invoices SET status=?1, updated_at=?2 WHERE id=?3",
        params![status, now, id],
    )?;
    Ok(n > 0)
}

pub fn delete_invoice(conn: &Connection, id: i64) -> Result<bool> {
    let n = conn.execute("DELETE FROM invoices WHERE id = ?1", params![id])?;
    Ok(n > 0)
}
