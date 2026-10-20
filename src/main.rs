mod bookmark;

use std::env;
use chrono;
use rusqlite::{self, Error, ffi, params};

fn main() {

    let conn = rusqlite::Connection::open("bookmarks.db").unwrap();

    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS bookmark (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            target TEXT NOT NULL,
            created_at DATETIME NOT NULL,
            updated_at DATETIME NOT NULL
        )",
        [], 
    );
    
    let args: Vec<String> = env::args().collect();
    let action: &String = &args[1];

    match action.as_str() {
        "-a" | "--add" => save_bookmark(&conn, &args[2].as_str(), &args[3].as_str()),
        "-d" | "--delete" => delete_bookmark(&conn, &args[2].as_str()),
        "-u" | "--update" => update_bookmark(&args[2].as_str(), &args[3].as_str()),
        "-l" | "--list" => list_bookmarks(&conn),
        "-o" | "--open" => open_bookmark(&conn, &args[2].as_str()),
        _ => print!("Hello")

    }
}

fn save_bookmark(conn: &rusqlite::Connection, name: &str, target: &str) {
    let current_time = chrono::Utc::now();
    let result = conn.execute("INSERT INTO bookmark (name, target, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)", rusqlite::params![name, target, current_time, current_time]);

    match result {
        Ok(_num) => println!("Saved {} for target {}", name, target),
        Err(Error::SqliteFailure(ffi::Error { code: ffi::ErrorCode::ConstraintViolation, extended_code: 2067 }, _msg)) => println!("Cannot save {} because it already exists for another target", name),
        Err(_) => println!("Unknown error")
    }
}

fn delete_bookmark(conn: &rusqlite::Connection, name: &str) {
    let result = conn.execute("DELETE FROM bookmark WHERE name = ?1", params![name]);

    match result {
        Ok(num) if num > 0 => println!("Deleted {}", name),
        Ok(_) => println!("Could not delete {}: bookmark not found", name),
        Err(_) => println!("Could not delete {}", name)
    }
}

fn update_bookmark(_name: &str, _target: &str) {

}

fn list_bookmarks(conn: &rusqlite::Connection) {
    let mut stmt = conn.prepare("SELECT id, name, target, created_at, updated_at FROM bookmark").unwrap();
    
    let rows = stmt.query_map([], |row| {
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        let target: String = row.get(2)?;
        let created_at: String = row.get(3)?;
        let updated_at: String = row.get(4)?;
        Ok((id, name, target, created_at, updated_at)) 
    }).unwrap();

    for row in rows {
        println!("{:?}", row.unwrap()); 
    }
}

fn open_bookmark(conn: &rusqlite::Connection, name: &str) {
    let mut stmt = conn.prepare("SELECT target FROM bookmark WHERE name = ?1").unwrap();
    
    let result: Result<String, _> = stmt.query_row([name], |row| row.get(0));

    match result {
        Ok(target) => {
            if let Err(e) = webbrowser::open(&target) {
                println!("Failed to open browser: {}", e);
            }
        },
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            println!("Bookmark '{}' not found.", name);
        },
        Err(_) => println!("Something went wrong")
    }
}
