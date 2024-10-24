use models::{parse_col, Game, Rom};
use query::{Column, Query};
use rusqlite::{OpenFlags, Params, PrepFlags};
use std::{fmt::Write, path::Path};

pub extern crate rusqlite;
pub mod models;
pub mod query;

#[derive(Debug)]
pub struct Connection {
    conn: rusqlite::Connection,
}

impl Connection {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            conn: rusqlite::Connection::open_with_flags(
                path,
                OpenFlags::SQLITE_OPEN_NO_MUTEX | OpenFlags::SQLITE_OPEN_READ_ONLY,
            )?,
        })
    }

    pub fn prepare<C: IntoIterator<Item = Column>>(
        &self,
        query: Query<C>,
        flags: PrepFlags,
    ) -> Result<Statement<'_>, rusqlite::Error> {
        let sql = build_sql(query);
        println!("{sql}");
        let stmt = self.conn.prepare_with_flags(&sql, flags)?;
        return Ok(Statement { stmt });
    }
}

#[derive(Debug)]
pub struct Statement<'a> {
    stmt: rusqlite::Statement<'a>,
}

impl<'a> Statement<'a> {
    pub fn query<S: for<'b> From<&'b str>>(
        &mut self,
        params: impl Params,
    ) -> Result<Rows<S>, rusqlite::Error> {
        Ok(Rows {
            prev: None,
            iter: self.stmt.query(params)?,
        })
    }
}

pub struct Rows<'a, S> {
    prev: Option<Game<S>>,
    iter: rusqlite::Rows<'a>,
}

impl<'a, S: for<'b> From<&'b str>> Iterator for Rows<'a, S> {
    type Item = Result<Game<S>, rusqlite::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! tri {
            ($e:expr) => {
                match $e {
                    Ok(x) => x,
                    Err(e) => return Some(Err(e)),
                }
            };
        }

        let mut game = match self.prev.take() {
            Some(prev) => prev,
            None => {
                let row = match self.iter.next().transpose()? {
                    Ok(row) => row,
                    Err(e) => return Some(Err(e)),
                };

                let id = unsafe { tri!(parse_col::<u32>(row, "id")).unwrap_unchecked() };
                let mut game = tri!(Game::<S>::parse(&row, id));
                game.roms.extend(tri!(Rom::<S>::parse(&row)));
                game
            }
        };

        while let Some(row) = tri!(self.iter.next()) {
            let id = unsafe { tri!(parse_col::<u32>(row, "id")).unwrap_unchecked() };
            let rom = tri!(Rom::<S>::parse(&row));

            if id != game.id {
                let mut curr = tri!(Game::<S>::parse(row, id));
                curr.roms.extend(rom);
                self.prev = Some(curr);
                break;
            } else {
                game.roms.extend(rom);
            }
        }

        return Some(Ok(game));
    }
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
    let mut res = format!(
        "SELECT {fields} FROM games {joins} WHERE {} ORDER BY games.id",
        r#where
    );

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
