use crate::db;
use anyhow::Result;
use rusqlite::Connection;

pub fn run(conn: &Connection) -> Result<()> {
    db::init_db(conn)?;
    println!("ERP database initialized successfully.");
    Ok(())
}
