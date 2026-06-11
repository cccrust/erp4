use rusqlite::Connection;
use anyhow::Result;
use crate::db;

pub fn run(conn: &Connection) -> Result<()> {
    db::init_db(conn)?;
    println!("ERP database initialized successfully.");
    Ok(())
}
