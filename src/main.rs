// Imports
use std::process;

// Modules
mod db;

fn main() {
    let conn = match db::init_db() {
        Ok(conn) => {
            println!("Database initialized and sample data inserted.");
            conn
        }
        Err(e) => {
            eprintln!("Error initializing the database: {}", e);
            process::exit(1); // Exit with status 1 on error
        }
    };
    
    match db::fetch_all_items(&conn) {
        Ok(items) => {
            for item in items {
                println!("Fetched item: {:?}", item);
            }
        }
        Err(e) => {
            eprintln!("Error fetching items: {}", e);
            process::exit(1); // Exit with status 1 on error
        }
    }
}

