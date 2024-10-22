use ouroboros::self_referencing;
use query::{Column, Query};
use rusqlite::{OpenFlags, PrepFlags, Statement};
use std::{
    fmt::Write,
    path::Path,
    sync::{Condvar, Mutex},
};

pub mod models;
pub mod query;

pub struct Pool {
    cond: Condvar,
    conns: Box<[Mutex<Connection>]>,
}

impl Pool {
    pub fn new(path: impl AsRef<Path>, size: usize) -> Result<Self, rusqlite::Error> {
        let conns = (0..size)
            .map(|_| {
                let conn = rusqlite::Connection::open_with_flags(
                    path.as_ref(),
                    OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
                )?;
                Ok(Mutex::new(Connection::new(conn, |_| Vec::new())))
            })
            .collect::<Result<Box<[_]>, rusqlite::Error>>()?;

        return Ok(Self {
            cond: Condvar::new(),
            conns,
        });
    }

    pub fn prepare<C: IntoIterator<Item = Column>>(
        &mut self,
        query: Query<C>,
        flags: PrepFlags,
    ) -> Result<Statement<'_>, rusqlite::Error> {
        let sql = build_sql(query);
        return self.conn.prepare_with_flags(&sql, flags);
    }
}

#[self_referencing]
struct Connection {
    conn: rusqlite::Connection,
    #[borrows(conn)]
    #[covariant]
    stmts: Vec<Statement<'this>>,
}

fn build_sql<C: IntoIterator<Item = Column>>(
    Query {
        cols,
        r#where,
        limit,
        offset,
    }: Query<C>,
) -> String {
    let mut fields = vec!["games.id"];
    let mut joins = Vec::new();
    cols.into_iter()
        .for_each(|col| col.transform(&mut fields, &mut joins));

    let fields = fields.join(",");
    let joins = joins.join(",");
    let mut res = format!("SELECT {fields} FROM games {joins} WHERE {}", r#where);

    if let Some(limit) = limit {
        res.push_str(" LIMIT ");
        res.write_fmt(format_args!("{limit}")).unwrap();
    }

    if let Some(offset) = offset {
        res.push_str(" OFFSET ");
        res.write_fmt(format_args!("{offset}")).unwrap();
    }

    return res;
}
