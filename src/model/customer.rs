use rusqlite::{params, Connection};
use anyhow::Result;
use chrono::Local;

#[derive(Debug, Clone)]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn create_customer(conn: &Connection, name: &str, email: Option<&str>, phone: Option<&str>, address: Option<&str>) -> Result<i64> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO customers (name, email, phone, address, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![name, email, phone, address, now, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_customers(conn: &Connection) -> Result<Vec<Customer>> {
    let mut stmt = conn.prepare("SELECT id, name, email, phone, address, created_at, updated_at FROM customers ORDER BY id")?;
    let rows = stmt.query_map([], |row| {
        Ok(Customer {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            phone: row.get(3)?,
            address: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;
    let mut customers = Vec::new();
    for row in rows {
        customers.push(row?);
    }
    Ok(customers)
}

pub fn get_customer(conn: &Connection, id: i64) -> Result<Option<Customer>> {
    let mut stmt = conn.prepare("SELECT id, name, email, phone, address, created_at, updated_at FROM customers WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(Customer {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            phone: row.get(3)?,
            address: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;
    match rows.next() {
        Some(Ok(c)) => Ok(Some(c)),
        _ => Ok(None),
    }
}

pub fn update_customer(conn: &Connection, id: i64, name: Option<&str>, email: Option<&str>, phone: Option<&str>, address: Option<&str>) -> Result<bool> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let existing = get_customer(conn, id)?;
    let c = match existing {
        Some(c) => c,
        None => return Ok(false),
    };
    let name = name.unwrap_or(&c.name);
    let email = email.or(c.email.as_deref());
    let phone = phone.or(c.phone.as_deref());
    let address = address.or(c.address.as_deref());
    conn.execute(
        "UPDATE customers SET name=?1, email=?2, phone=?3, address=?4, updated_at=?5 WHERE id=?6",
        params![name, email, phone, address, now, id],
    )?;
    Ok(true)
}

pub fn delete_customer(conn: &Connection, id: i64) -> Result<bool> {
    let n = conn.execute("DELETE FROM customers WHERE id = ?1", params![id])?;
    Ok(n > 0)
}
