use crate::bot_core::constants::DB_FILE_NAME;

use std::path::Path;

use rusqlite::Connection;

pub struct RzDb {}

impl RzDb {
    pub fn get_connection() -> Connection {
        return Connection::open(DB_FILE_NAME).unwrap();
    }

    pub fn tx_execute(conn: &Connection, query: &str) {
        let tx = conn.unchecked_transaction();
        conn.execute(query, ());
        tx.expect("EXECUTE QUERY FAILED").commit();
    }

    pub fn is_exists() -> bool {
        return Path::new(DB_FILE_NAME).exists();
    }
}