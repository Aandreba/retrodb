use retrodb::{
    query::{Column, Query},
    Pool,
};
use rusqlite::PrepFlags;

#[test]
fn test() -> Result<(), rusqlite::Error> {
    let conn = Pool::new("../libretrodb.sqlite")?;
    let mut stmt = conn.prepare(
        Query::new([Column::Roms])
            .limit(1)
            .r#where("games.full_name LIKE $1"),
        PrepFlags::empty(),
    )?;

    let mut res = stmt.query(["%Mario Kart DS%"])?;
    println!("{:?}", res.next());

    return Ok(());
}
