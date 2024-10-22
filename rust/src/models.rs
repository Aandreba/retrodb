use rusqlite::{
    types::{FromSql, ValueRef},
    Error, Row,
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Game<S> {
    pub id: u32,
    pub serial_id: Option<S>,
    pub developer: Option<S>,
    pub publisher: Option<S>,
    pub rating: Option<S>,
    pub users: Option<u32>,
    pub franchise: Option<S>,
    pub release_year: Option<u32>,
    pub release_month: Option<u8>,
    pub region: Option<S>,
    pub genre: Option<S>,
    pub display_name: Option<S>,
    pub full_name: Option<S>,
    pub platform: Option<Platform<S>>,
    pub roms: Vec<Rom<S>>,
}

impl<S: for<'a> From<&'a str>> Game<S> {
    pub(crate) fn parse(row: &Row, id: u32) -> Result<Self, rusqlite::Error> {
        return Ok(Self {
            id,
            serial_id: parse_str(row, "serial_id")?,
            developer: parse_str(row, "developer")?,
            publisher: parse_str(row, "publisher")?,
            rating: parse_str(row, "rating")?,
            users: parse_col(row, "users")?,
            franchise: parse_str(row, "franchise")?,
            release_year: parse_col(row, "release_year")?,
            release_month: parse_col(row, "release_month")?,
            region: parse_str(row, "region")?,
            genre: parse_str(row, "genre")?,
            display_name: parse_str(row, "display_name")?,
            full_name: parse_str(row, "full_name")?,
            platform: Platform::parse(row)?,
            roms: Vec::new(),
        });
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rom<S> {
    pub id: u32,
    pub name: Option<S>,
    pub crc: Option<u32>,
}

impl<S: for<'a> From<&'a str>> Rom<S> {
    pub(super) fn parse(row: &Row) -> Result<Option<Self>, rusqlite::Error> {
        let Some(id) = parse_col::<u32>(row, "rom_id")? else {
            return Ok(None);
        };

        return Ok(Some(Self {
            id,
            name: parse_str(row, "rom_name")?,
            crc: parse_col(row, "rom_crc")?,
        }));
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Platform<S> {
    pub name: S,
    pub manufacturer: Option<S>,
}

impl<S: for<'a> From<&'a str>> Platform<S> {
    fn parse(row: &Row) -> Result<Option<Self>, rusqlite::Error> {
        let Some(name) = parse_str::<S>(row, "platform")? else {
            return Ok(None);
        };

        return Ok(Some(Self {
            name,
            manufacturer: parse_str(row, "manufacturer")?,
        }));
    }
}

#[inline]
pub(crate) fn parse_col<'a, T: FromSql>(
    row: &'a Row,
    name: &str,
) -> Result<Option<T>, rusqlite::Error> {
    unsafe { Ok(<Option<T> as FromSql>::column_result(get_col(row, name)?).unwrap_unchecked()) }
}

#[inline]
pub(crate) fn parse_str<'a, S: From<&'a str>>(
    row: &'a Row,
    name: &str,
) -> Result<Option<S>, rusqlite::Error> {
    Ok(unsafe {
        get_col(row, name)?
            .as_str_or_null()
            .unwrap_unchecked()
            .map(S::from)
    })
}

#[inline]
fn get_col<'a>(row: &'a Row, name: &str) -> Result<ValueRef<'a>, rusqlite::Error> {
    return match row.get_ref(name) {
        Ok(value) => Ok(value),
        Err(Error::InvalidColumnName(_)) => Ok(ValueRef::Null),
        Err(e) => return Err(e),
    };
}
