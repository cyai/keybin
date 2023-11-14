use rusqlite::{Connection, Result, params};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref CONNECTION: Arc<Mutex<Connection>> = {
        Arc::new(Mutex::new(
            Connection::open("secrets.db").expect("Failed to open database")
        ))
    };
}

pub async fn create_db() -> Result<()> {
    let conn = CONNECTION.lock().expect("Failed to obtain lock");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS secrets (
            name TEXT NOT NULL PRIMARY KEY,
            id TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub async fn insert_secret(name: &str, id: &str) -> Result<()> {
    let conn = CONNECTION.lock().expect("Failed to obtain lock");
    conn.execute(
        "INSERT INTO secrets (name, id) VALUES (?1, ?2)",
        &[name, id],
    )?;
    Ok(())
}

pub async fn get_secret_id(name: &str) -> Result<Option<String>, rusqlite::Error> {
    let conn = CONNECTION.lock().expect("Failed to obtain lock");

    let mut stmt = conn.prepare("SELECT id FROM secrets WHERE name = ?1")?;

    let mut rows = stmt.query(params![name])?;

    if let Some(row) = rows.next()? {
        let id: String = row.get(0)?;
        Ok(Some(id))
    } else {
        Ok(None)
    }
}
