use rusqlite::Connection;

use crate::{logs, records::Record};

pub struct Database {
    conn: Connection,
}
impl Database {
    /// Create the connectino to the database, and create a results table if it doesn't exist
    pub fn new() -> Self {
        logs("Creating database connection.");
        let conn = Connection::open("NetMetrics.db").unwrap();

        logs("Creating results table.");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS results (
            download INTEGER,
            upload INTEGER,
            ping INTEGER
            inserted_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
            [],
        )
        .unwrap();

        Self { conn }
    }

    pub fn insert_record(&self, record: Record) {
        logs("Inserting record into database.");
        self.conn
            .execute(
                "INSERT INTO results (download, upload, ping) VALUES (?1, ?2, ?3)",
                [
                    record.download as i64,
                    record.upload as i64,
                    record.ping as i64,
                ],
            )
            .unwrap();
    }
}
