pub mod program;

use rusqlite::{Connection, Result};

// Open a connection to the SQLite database file
pub fn open_connection() -> Result<Connection> {
    Connection::open("search_help.sqlite")
}
