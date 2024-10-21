use crate::Connection;
use rusqlite::PrepFlags;

pub mod filter;

#[derive(Debug)]
pub struct Query<'a> {
    distinct: bool,
    r#where: &'a str,
    limit: Option<u64>,
    offset: Option<u64>,
}

impl<'a> Query<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            distinct: false,
            r#where: "",
            limit: None,
            offset: None,
        }
    }

    pub fn build(self, conn: &Connection, flags: PrepFlags) {
        conn.conn.
        // conn.conn.prepare_with_flags(sql, flags)
        todo!()
    }
}

impl<'a> Default for Query<'a> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Column {
    SerialId,
    Developer,
    Publisher,
    Rating,
    Users,
    Franchise,
    ReleaseYear,
    ReleaseMonth,
    Region,
    Genre,
    DisplayName,
    FullName,
    Platform,
    Roms,
}
