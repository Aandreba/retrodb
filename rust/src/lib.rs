use rusqlite::OpenFlags;
use std::path::Path;

pub mod models;
pub mod tables;

pub struct Connection {
    conn: rusqlite::Connection,
}

impl Connection {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, rusqlite::Error> {
        let conn = rusqlite::Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        return Ok(Self { conn });
    }
}
