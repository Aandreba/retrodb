use retrodb::{
    query::{Column, Query},
    Connection,
};
use rusqlite::PrepFlags;

#[test]
fn test() -> Result<(), rusqlite::Error> {
    let conn = Connection::open("../libretrodb.sqlite")?;
    let mut stmt = conn.prepare(
        Query::new([Column::Roms]).r#where("games.full_name LIKE $1"),
        PrepFlags::empty(),
    )?;

    let mut rows = stmt.query::<String>(["%Mario Kart%"])?;
    while let Some(game) = rows.next().transpose()? {
        println!("{game:#?}");
    }

    return Ok(());
}
