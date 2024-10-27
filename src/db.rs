// imports
use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Item {
    id: i32,
    name: String,
}

// Define SQL statements as constants
const CREATE_TABLE_SQL: &str = "CREATE TABLE IF NOT EXISTS items (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
)";

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("items.db")?;
    
    conn.execute(CREATE_TABLE_SQL, [])?;
    return Ok(conn);
}

pub fn fetch_all_items(conn: &Connection) -> Result<Vec<Item>> {
    let mut stmt = conn.prepare("SELECT id, name FROM items")?;
    
    let item_iter = stmt.query_map([], |row| {
        Ok(Item {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    let items: Result<Vec<Item>, rusqlite::Error> = item_iter.collect();
    return items;
}

